import { deleteImage, displayImageNow, getImages } from "@/apis/api";
import { useMutation, useQueryClient, useSuspenseQuery } from "@tanstack/react-query";
import { Suspense } from "react";
import { ImageGridSkeleton } from "./ImageGridSkeleton";
import { Button } from "./ui/button";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger } from "./ui/alert-dialog";
import { ComputerIcon } from "lucide-react";
import { toast } from "sonner";


export const ImageGrid = () => {

    return <div className="flex flex-wrap gap-4 justify-center w-full">
        <Suspense fallback={<ImageGridSkeleton />}>
            <ImageGridItems />
        </Suspense>
    </div>;
};

const ImageGridItems = () => {
    const queryClient = useQueryClient();

    const images = useSuspenseQuery({
        queryKey: ['images'],
        queryFn: getImages
    });

    const displayNowMutation = useMutation({
        mutationFn: async (imageId: string) => {
            // Call your API to display the image now
            await displayImageNow(imageId);
        },
        onError: (error) => {
            toast.error("Failed to display image on frame.", { description: (error as Error).message });
        },
        onSuccess: () => {
            toast.success("Image will be displayed on the frame shortly.");
        }
    })

    const deleteImageMutation = useMutation({
        mutationFn: async (imageId: string) => {
            // Call your API to delete the image
            await deleteImage(imageId);
        },
        onSuccess: () => {
            // Invalidate and refetch
            queryClient.invalidateQueries({ queryKey: ['images'] });
        },
        onError: (error) => {
            toast.error("Failed to delete image", { description: (error as Error).message });
        }
    });

    return (<>
        {images.data.map((image) => (
            <div key={image.filePath} className="w-48 h-32 relative rounded-lg overflow-hidden">

                <AlertDialog>
                    <AlertDialogTrigger asChild>
                        <Button variant="outline" className="absolute top-2 right-2">x</Button>
                    </AlertDialogTrigger>
                    <AlertDialogContent>
                        <AlertDialogHeader>
                            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
                            <AlertDialogDescription>
                                This action cannot be undone. This will permanently delete this
                                image from the system.
                            </AlertDialogDescription>
                        </AlertDialogHeader>
                        <AlertDialogFooter>
                            <AlertDialogCancel>Cancel</AlertDialogCancel>
                            <AlertDialogAction onClick={() => deleteImageMutation.mutate(image.fileNameWithExtension)}>Delete</AlertDialogAction>
                        </AlertDialogFooter>
                    </AlertDialogContent>
                </AlertDialog>

                <AlertDialog>
                    <AlertDialogTrigger asChild>
                        <Button variant="outline" className="absolute top-2 left-2"><ComputerIcon /></Button>
                    </AlertDialogTrigger>
                    <AlertDialogContent>
                        <AlertDialogHeader>
                            <AlertDialogTitle>Show this on the frame now?</AlertDialogTitle>
                            <AlertDialogDescription>
                                This will immediately display this image on the frame. It will
                                be replaced the next time the daily update occurs.
                            </AlertDialogDescription>
                        </AlertDialogHeader>
                        <AlertDialogFooter>
                            <AlertDialogCancel>Cancel</AlertDialogCancel>
                            <AlertDialogAction onClick={() => displayNowMutation.mutate(image.filePath)}>Show</AlertDialogAction>
                        </AlertDialogFooter>
                    </AlertDialogContent>
                </AlertDialog>

                <img
                    src={image.url}
                    alt={`Image`}
                    className="w-full h-full object-cover"
                    loading="lazy"
                />
            </div>
        ))}
    </>
    )
};