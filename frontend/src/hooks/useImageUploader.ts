import { useCallback, useEffect, useState } from "react";

export type ImageValue = File | string;

type ImageItem = {
    id: string;
    file?: File;
    url?: string;
    preview: string;
};

type UseImageUploaderOptions = {
    max?: number;
    imageValue?: ImageValue[];
    onChange?: (value: ImageValue[]) => void;
};

// todo: Create a imageCropper
// - Create a cache for default image to be able on reset or re-crop
// - Create a new ID on cropped-image and replace in imagesValue(Main container)
// - On reset or not doing anything will not count as crop
// -- function check is image has been crop yet?

export function useImageUploader({
    max = 10,
    imageValue,
    onChange
}: UseImageUploaderOptions) {
    const [images, setImages] = useState<ImageItem[]>([]);
    
    const syncImages = useCallback((items: ImageItem[]) => {
        setImages(items);
    }, []);
    
    const updateImages = useCallback((items: ImageItem[]) => {
        setImages(items);
        onChange?.(
            items.map(item => item.file ?? item.url!)
        );
    }, [onChange]);

    const addFiles = useCallback((files: FileList | File[]) => {
        const remaining = max - images.length;
        if (remaining <= 0) return;

        const selected = Array.from(files).slice(0, remaining);

        updateImages([
            ...images,
            ...selected.map(file => ({
                id: crypto.randomUUID(),
                file,
                preview: URL.createObjectURL(file)
            }))
        ]);
    }, [images, max]);

    const removeImage = useCallback((id: string) => {
        const image = images.find(x => x.id === id);

        if (image?.file) {
            URL.revokeObjectURL(image.preview);
        }

        updateImages(images.filter(x => x.id !== id));
    }, [images]);

    useEffect(() => {
    if (!imageValue) return;

    syncImages(
        imageValue.map(v =>
            typeof v === "string"
                ? {
                    id: crypto.randomUUID(),
                    url: v,
                    preview: `/cdn/${v}`
                }
                : {
                    id: crypto.randomUUID(),
                    file: v,
                    preview: URL.createObjectURL(v)
                }
        )
    );
}, [imageValue, syncImages]);

    return {
        images,
        addFiles,
        removeImage
    };
}