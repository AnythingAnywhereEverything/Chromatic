import s from "@styles/ui/chromatic/imageuploader2.module.scss"
import { useImageUploader } from "@/hooks/useImageUploader";
import React, { useRef } from "react";

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

export {
    ImageUploader2, 
    ContainerPreview
}