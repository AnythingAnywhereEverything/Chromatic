"use client";

import React, { useEffect, useRef, useState } from "react";
import { Button } from "./button";
import s from "@styles/ui/Chormatic/imageuploader.module.scss";
import { FieldError, Icon } from "../chromaticUI";

import {
    DndContext,
    closestCenter,
    PointerSensor,
    useSensor,
    useSensors
} from "@dnd-kit/core";

import {
    SortableContext,
    useSortable,
    rectSortingStrategy,
    arrayMove
} from "@dnd-kit/sortable";

import { CSS } from "@dnd-kit/utilities";
import Image from "next/image";
import { IoClose } from "react-icons/io5";
import { HiOutlinePlus } from "react-icons/hi";

type ImageValue = File | string;

type ImageItem = {
    id: string;
    file?: File;
    url?: string;
    preview: string;
};

type ImageUploaderProps = {
    id?: string;
    min?: number;
    max?: number;
    value?: ImageValue[];
    accept?: string;
    onChange?: (files: ImageValue[]) => void;
};

/* ---------- Sortable Item ---------- */

const SortableImage = React.memo(({
    item,
    index,
    onRemove
}: { item: ImageItem; index: number; onRemove: () => void }) => {
    const {
        attributes,
        listeners,
        setNodeRef,
        transform,
        transition,
        isDragging
    } = useSortable({ id: item.id });

    const style = {
        transform: CSS.Transform.toString(transform),
        transition,
        zIndex: isDragging ? 1000 : undefined,
        position: isDragging ? "relative" as const : undefined
    };

    return (
        <div
            ref={setNodeRef}
            style={style}
            {...attributes}
            {...listeners}
            className={s.imageWrapper}
        >
            <Image
                src={item.preview}
                alt=""
                fill
                style={{
                    width: "100%",
                    height: "100%",
                    objectFit: "cover",
                    pointerEvents: "none"
                }}
            />

            <Button
                type="button"
                variant={"outline"}
                size={"icon-sm"}
                onPointerDown={(e) => e.stopPropagation()}
                onClick={onRemove}
                className={s.removeButton}
            >
            <IoClose/>
            </Button>
        </div>
    );
});

const ImageUploader: React.FC<ImageUploaderProps> = ({
    id = "",
    min = 0,
    max = 5,
    value,
    accept = "image/*",
    onChange
}) => {

    const fileInputRef = useRef<HTMLInputElement>(null);
    const [images, setImages] = useState<ImageItem[]>([]);

    const sensors = useSensors(
        useSensor(PointerSensor)
    );

    /* ---------- helpers ---------- */

    const updateState = (items: ImageItem[]) => {
        setImages(items);

        const output: ImageValue[] = items.map(i => {
            if (i.file) return i.file;
            return i.url!;
        });

        onChange?.(output);
    };

    /* ---------- add files ---------- */

    const handleFiles = (e: React.ChangeEvent<HTMLInputElement>) => {
        const files = e.target.files;
        if (!files) return;

        const remaining = max - images.length;
        if (remaining <= 0) return;

        const selected = Array.from(files).slice(0, remaining);

        const newItems: ImageItem[] = selected.map(file => ({
            id: crypto.randomUUID(),
            file,
            preview: URL.createObjectURL(file)
        }));

        updateState([...images, ...newItems]);

        e.target.value = "";
    };

    const handleAddClick = () => {
        fileInputRef.current?.click();
    };

    /* ---------- remove ---------- */

    const handleRemove = (index: number) => {
        const target = images[index];

        if (target.file) {
            URL.revokeObjectURL(target.preview);
        }

        updateState(
            images.filter((_, i) => i !== index)
        );
    };

    /* ---------- drag ---------- */

    const handleDragEnd = (event: any) => {
        const { active, over } = event;
        if (!over || active.id === over.id) return;

        const oldIndex = images.findIndex(i => i.id === active.id);
        const newIndex = images.findIndex(i => i.id === over.id);

        updateState(
            arrayMove(images, oldIndex, newIndex)
        );
    };

    /* ---------- sync value ---------- */

    useEffect(() => {
        if (!value) return;

        setImages(() => {
            const mapped: ImageItem[] = value.map(v => {
                if (typeof v === "string") {
                    return {
                        id: crypto.randomUUID(),
                        url: v,
                        preview: `/cdn/${v}`
                    };
                }

                return {
                    id: crypto.randomUUID(),
                    file: v,
                    preview: URL.createObjectURL(v)
                };
            });

            return mapped;
        });

    }, [value]);

    return (
        <div>

            <input
                id={id}
                ref={fileInputRef}
                type="file"
                accept={accept}
                multiple
                hidden
                onChange={handleFiles}
            />

            <DndContext
                sensors={sensors}
                collisionDetection={closestCenter}
                onDragEnd={handleDragEnd}
            >
                <SortableContext
                    items={images.map(i => i.id)}
                    strategy={rectSortingStrategy}
                >
                    <div style={{ display: "flex", gap: 12, flexWrap: "wrap" }}>
                        {images.map((img, index) => (
                            <SortableImage
                                key={img.id}
                                item={img}
                                index={index}
                                onRemove={() => handleRemove(index)}
                            />
                        ))}

                        {images.length < max && (
                            <Button
                                type="button"
                                onClick={handleAddClick}
                                variant={"outline"}
                                className={s.addButton}
                                asChild
                            >
                                <HiOutlinePlus/>
                            </Button>
                        )}
                    </div>
                </SortableContext>
            </DndContext>

            {images.length < min && (
                <FieldError style={{ marginTop: 8 }}>
                    Minimum {min} image(s) required
                </FieldError>
            )}

        </div>
    );
};

export default React.memo(ImageUploader);