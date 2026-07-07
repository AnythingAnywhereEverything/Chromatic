import s from "@styles/ui/Chormatic/imageuploader2.module.scss"
import { useImageUploader } from "@/hooks/useImageUploader";
import React, { useRef } from "react";

import style from "@styles/ui/Chormatic/imageuploader2.module.scss"
import { Button, Dialog } from "../chromaticUI";
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
        <div className={style["createImageContainer"]}>
            {!!images && (
                <div className={`${style.gallery} ${style[galleryClass]}`}>
                    <Button
                    variant={'default'} 
                    className={style["editButton"]}
                    onClick={() => {}}
                    >
                        Edit all image
                    </Button>
                    {visibleImages.map((image, index) => (
                        <div
                            key={image.id ?? `${index}`}
                            className={s.item}
                        >
                            <img src={image.preview} alt="" className={style.image} />
                            <button
                                type="button"
                                className={style.removeImage}
                                onClick={() => onDelete(image.id)}
                            >
                                ✕
                            </button>
                            
                            {count > 4 && index === 3 && (
                                <div className={style.overlay}>
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
            
        </Dialog>
    )
}
export {
    ImageUploader2, 
    ContainerPreview,
    ContainerPreview2
}