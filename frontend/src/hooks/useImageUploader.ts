import { useCallback, useEffect,useRef, useState } from "react";

export type ImageValue = File | string;

export type ImageItem = {
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
// IHATEIT
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
    // * id -> original item, set the first time that id is cropped
    // ! resync doesn't clear old ids, map can grow if imageValue changes a lot
    const defaultCacheRef = useRef<Map<string, ImageItem>>(new Map());

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

        const defaultItem = defaultCacheRef.current.get(id);
        // * cropped image also holds a cached original blob, revoke that too
        if (defaultItem && defaultItem.id !== id && defaultItem.file) {
            URL.revokeObjectURL(defaultItem.preview);
        }
        defaultCacheRef.current.delete(id);

        updateImages(images.filter(x => x.id !== id));
    }, [images]);

    const replaceImage = useCallback((id: string, file: File) => {
        const current = images.find(x => x.id === id);
        if (!current) return;

        // * nothing cached yet on first crop, current becomes the default
        const defaultItem = defaultCacheRef.current.get(id) ?? current;

        if (current.file && current.id !== defaultItem.id) {
            URL.revokeObjectURL(current.preview);
        }

        const newItem = {
            id: crypto.randomUUID(),
            file,
            preview: URL.createObjectURL(file)
        };

        defaultCacheRef.current.delete(id);
        defaultCacheRef.current.set(newItem.id, defaultItem);

        updateImages(images.map(x => (x.id === id ? newItem : x)));
    }, [images]);

    const resetImage = useCallback((id: string) => {
        const defaultItem = defaultCacheRef.current.get(id);
        // * nothing cached, or already showing the default
        if (!defaultItem || defaultItem.id === id) return;

        const current = images.find(x => x.id === id);
        if (!current) return;

        if (current.file) {
            URL.revokeObjectURL(current.preview);
        }

        defaultCacheRef.current.delete(id);

        updateImages(images.map(x => (x.id === id ? defaultItem : x)));
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
        removeImage,
        replaceImage,
        resetImage
    };
}