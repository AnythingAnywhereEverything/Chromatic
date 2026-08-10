import { useEffect, useRef, useState } from "react";
import { Dialog, DialogContent, DialogTrigger } from "../dialogue";
import { ImageItem } from "@/hooks/useImageUploader";
import ReactCrop, { type Crop, type PixelCrop } from "react-image-crop";

interface ImageCropProps{
    id: string;
    media: string;
    onReplace: (id: string, file: File) => void;
    onReset: (id: string) => void;
}
export async function cropToFile(image: HTMLImageElement, crop: PixelCrop): Promise<File> {
    const canvas = document.createElement("canvas");
    const scaleX = image.naturalWidth / image.width;
    const scaleY = image.naturalHeight / image.height;

    canvas.width = crop.width;
    canvas.height = crop.height;

    const ctx = canvas.getContext("2d");
    if (!ctx) throw new Error("canvas context unavailable");

    ctx.drawImage(
        image,
        crop.x * scaleX,
        crop.y * scaleY,
        crop.width * scaleX,
        crop.height * scaleY,
        0,
        0,
        crop.width,
        crop.height
    );

    const blob = await new Promise<Blob>((resolve, reject) => {
        // * jpeg output, change the mime type if you need png transparency
        canvas.toBlob(b => (b ? resolve(b) : reject(new Error("toBlob failed"))), "image/jpeg");
    });

    return new File([blob], "cropped.jpg", { type: blob.type });
}
// * put trigger in the image
const ImageCropper:React.FC<ImageCropProps> = ({ id, media, onReplace, onReset }) => {
    const imgRef = useRef(null);
    const [open, setOpen] = useState(false);
    const [crop, setCrop] = useState<Crop>();
    const [completedCrop, setCompletedCrop] = useState<PixelCrop>();
    
    const handleSave = async () => {
        if (!imgRef.current || !completedCrop) return;

        const file = await cropToFile(imgRef.current, completedCrop);
        onReplace(id, file);
        setOpen(false);
    };

    const handleReset = () => {
        onReset(id);
        setOpen(false);
    };
   return (
        <Dialog open={open} onOpenChange={setOpen}>
            <DialogTrigger asChild>
                <img src={media} alt="" style={{ cursor: "pointer" }} />
            </DialogTrigger>

            <DialogContent>
                <ReactCrop
                    crop={crop}
                    onChange={(_, percentCrop) => setCrop(percentCrop)}
                    
                    onComplete={setCompletedCrop}
                >
                    <img ref={imgRef} src={media} alt="" />
                </ReactCrop>

                <button onClick={handleSave} disabled={!completedCrop}>
                    Save
                </button>
                <button onClick={handleReset}>Reset</button>
            </DialogContent>
        </Dialog>
    );
}

export default ImageCropper;