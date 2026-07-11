import s from "@styles/ui/chromatic/imageuploader2.module.scss"
import { useImageUploader } from "@/hooks/useImageUploader";
import React, { useRef } from "react";

import { Button, Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from "../chromaticUI";
type ImageItem = {
    id: string;
    file?: File;
    url?: string;
    preview: string;
};

type ImageUploaderProps = {
    id?: string;
    accept?: string;
    uploader: ReturnType<typeof useImageUploader>;
    children?: React.ReactNode;
};

type ContainerPreviewProps = {
    images: ImageItem[];
    onDelete: (id: string) => void;
};

const ImageUploader2: React.FC<ImageUploaderProps> = ({
    id,
    accept = "image/*",
    uploader,
    children
}) => {
    const inputRef = useRef<HTMLInputElement>(null);

    const handleFiles = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (e.target.files) {
            uploader.addFiles(e.target.files);
        }

        e.target.value = "";
    };

    return (
        <>
            <input
                id={id}
                ref={inputRef}
                type="file"
                accept={accept}
                multiple
                hidden
                onChange={handleFiles}
            />

            <div onClick={() => inputRef.current?.click()}>
                {children}
            </div>
        </>
    );
};

const ContainerPreview: React.FC<ContainerPreviewProps> = ({
    images,
    onDelete
}) => {
    if (images.length === 0) {
        return null;
    }

    return (
        <div className={s.imageContainer}>
            {images.map((image) => (
                <div
                    key={image.id}
                    className={s.coverImage}
                >
                    <img
                        src={image.preview}
                        alt=""
                        className={s.image}
                    />

                    <button
                        type="button"
                        className={s.removeImage}
                        onClick={() => onDelete(image.id)}
                    >
                        ✕
                    </button>
                </div>
            ))}
        </div>
    );
};

const ContainerPreview2: React.FC<ContainerPreviewProps> = ({
    images,
    onDelete
}) => {
    const visibleImages = images.slice(0, 4);
    const count = images.length;
    const galleryClass = `gallery${Math.min(count, 4)}`;
    
    if (images.length === 0) {
        return null;
    }

    return (
        <div className={s["createImageContainer"]}>
            
            {!!images && (
                <div className={`${s.gallery} ${s[galleryClass]}`}>
                    <AllImagePreview
                    images={images}
                    onDelete={onDelete}
                    />
                    {visibleImages.map((image, index) => (
                        <div
                            key={image.id ?? `${index}`}
                            className={s.item}
                        >
                            <img src={image.preview} alt="" className={s.image} />

                            
                            {count > 4 && index === 3 && (
                                <div className={s.overlay}>
                                    +{count - 4}
                                </div>
                            )}
                        </div>
                    ))}
                </div>
                )}
        </div>
    );
};

const AllImagePreview: React.FC<ContainerPreviewProps> = ({
    images,
    onDelete    
}) => {
    return (
        <Dialog>
            <DialogTrigger asChild>
                <Button
                className={s["editButton"]}
                    >
                    Edit all image
                </Button>
            </DialogTrigger>
            <DialogContent className={s["imageContent"]}>
                <DialogTitle>Image editor</DialogTitle>
                <DialogDescription>
                    <div className={s["allImageContainer"]}>
                        {images.map((image, index) => (
                            <div
                            key={image.id ?? `${index}`}
                            className={s.item}
                            >
                                <img src={image.preview} alt="" className={s.image} />
                                <button
                                    className={s.removeImage}
                                    onClick={() => onDelete(image.id)}
                                    >
                                    ✕
                                </button>
                            </div>
                        ))}
                    </div>
                </DialogDescription>
            </DialogContent>
        </Dialog>
    )
}
// todo: Create a imageCropper
// - Create a cache for default image to be able on reset or re-crop
// - Create a new ID on cropped-image and replace in imagesValue(Main container)
// - On reset or not doing anything will not count as crop
// -- function check is image has been crop yet?

// function ImageCropper() {
//     const
// }


export {
    ImageUploader2, 
    ContainerPreview,
    ContainerPreview2
}