import { deleteImage, getImages } from "@/apis/api";
import { useMutation, useQueryClient, useSuspenseQuery } from "@tanstack/react-query";
import { Suspense } from "react";
import { ImageGridSkeleton } from "./ImageGridSkeleton";
import { Button } from "./ui/button";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger } from "./ui/alert-dialog";


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
            console.error("Error deleting image:", error);
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