import { BanIcon } from "lucide-react"
import { Empty, EmptyHeader, EmptyMedia, EmptyTitle, EmptyDescription, EmptyContent } from "./ui/empty"
import { Button } from "./ui/button"
import { Input } from "./ui/input";
import { Label } from "./ui/label";
import { Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from "./ui/dialog";
import { useMutation, useQueryClient } from "@tanstack/react-query";
// Update the path below to the correct location of your api file, for example:
import { uploadFile } from "../apis/api";

type ImageUploadedDialogContentProps = {
    reset: () => void;
};

const ImageUploadingDialogContent = () => (<><DialogHeader>
    <DialogTitle>Photo Uploading</DialogTitle>
    <DialogDescription>
        Your photo is currently being uploaded. Please wait...
    </DialogDescription>
</DialogHeader><DialogFooter>
        <DialogClose asChild>
            <Button variant="default">Cancel</Button>
        </DialogClose>
    </DialogFooter></>);

const ImageUploadedDialogContent: React.FC<ImageUploadedDialogContentProps> = ({ reset }) => (
    <>
        <DialogHeader>
            <DialogTitle>Photo Uploaded</DialogTitle>
            <DialogDescription>
                Your photo has been successfully uploaded and added
                to the rotation. You will start seeing it in your daily
                photos.
            </DialogDescription>
        </DialogHeader>
        <DialogFooter>
            <DialogClose asChild>
                <Button variant="default" onClick={reset}>Ok</Button>
            </DialogClose>
        </DialogFooter>
    </>
);

export const ImageUpload = () => {
    const queryClient = useQueryClient();

    const uploadImageMutation = useMutation({
        mutationFn: async (file: File) => {
            console.log("Uploading file:", file);
            await uploadFile(file);
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['images'] });
        },
        onError: (error) => {
            // "oh, oh!"
            console.error("Error uploading image:", error);
        }
    });

    const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        const file = event.target.files?.[0];
        console.log("Selected file:", file);
        if (file) {
            uploadImageMutation.mutate(file);
        }
    };

    return (
        <>
            <Empty>
                <EmptyHeader>
                    <EmptyMedia variant="icon">
                        <BanIcon />
                    </EmptyMedia>
                    <EmptyTitle>Add New Photo</EmptyTitle>
                    <EmptyDescription>
                        Photos will be randomly selected from what you upload. They will update every
                        day.
                    </EmptyDescription>
                </EmptyHeader>
                <EmptyContent>
                    <div className="flex gap-2">
                        <Label htmlFor="file-upload" className="m-0">
                            <Button asChild>
                                <span>
                                    Import Picture
                                    <Input type="file" accept="image/*" className="hidden" id="file-upload" onChange={handleFileChange} />
                                </span>
                            </Button>
                        </Label>
                    </div>
                </EmptyContent>
            </Empty>
            <Dialog open={uploadImageMutation.isPending || uploadImageMutation.isSuccess}>
                <DialogContent >
                    {uploadImageMutation.isPending && <ImageUploadingDialogContent />}
                    {uploadImageMutation.isSuccess && <ImageUploadedDialogContent reset={() => uploadImageMutation.reset()} />}
                </DialogContent>
            </Dialog>
        </>
    )
};