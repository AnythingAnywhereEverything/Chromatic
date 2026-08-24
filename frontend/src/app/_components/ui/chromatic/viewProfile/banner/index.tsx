import { useState, useRef, useEffect } from "react";
import getIdColor from "@lib/getIdColor";
import { Image } from "@/app/_components/ui/chromatic/Image";
import { OptimizationType } from "../../Image/type";
import style from "./banner.module.scss";
import { ImageEditor, imageUploadProps } from "../editor";
import {
    Dropdown,
    DropdownContent,
    DropdownItem,
    DropdownTrigger,
} from "../../dropdown";
import { FaPen } from "react-icons/fa";
import { ImageProcessor } from "@lib/cropImage";

export type BannerPayload = {
    file?: File;
    staticPreviewBlobUrl?: string | null;
    remove?: boolean;
};

interface BannerProps {
    userId: string;
    is_owner?: boolean;
    banner: string | null;
    blobUrl: string | null;
    thumbhash: string | null;
    onChange?: (payload: BannerPayload) => void;
    containerRef: React.RefObject<HTMLDivElement | null>;
}

interface BannerPreviewProps {
    userId: string;
    banner: string | null;
    blobUrl: string | null;
    thumbhash: string | null;
    bannerContainerRef?: React.RefObject<HTMLDivElement | null>;
}

function parseStaticImage (url: string ) {
    // remove the extension from the url
    if (url.startsWith("a_")) {
        const urlWithoutExtension = url.replace(/\.[^/.]+$/, "");
        return `${urlWithoutExtension}.png`;
    }
    return url;
}

const BannerPreview = ({
    userId,
    banner,
    blobUrl,
    thumbhash,
    bannerContainerRef,
}: BannerPreviewProps) => {
    if (!bannerContainerRef) {
        return null;
    }

    const [isInit, setIsInit] = useState(false);
    const [bannerInitWidth, setBannerInitWidth] = useState(600);
    const [bannerInitHeight, setBannerInitHeight] = useState(240);
    const [bannerContainerWidth, setBannerContainerWidth] = useState(600);
    const [bannerContainerHeight, setBannerContainerHeight] = useState(240);

    useEffect(() => {
        if (bannerContainerRef.current && !isInit) {
            setIsInit(true);
            setBannerInitWidth(
                Math.floor(bannerContainerRef.current.offsetWidth),
            );
            setBannerInitHeight(
                Math.floor((bannerContainerRef.current.offsetWidth * 2) / 5),
            );
        }

        const handleResize = () => {
            if (bannerContainerRef.current) {
                setBannerContainerWidth(
                    Math.floor(bannerContainerRef.current.offsetWidth),
                );
                setBannerContainerHeight(
                    Math.floor(
                        (bannerContainerRef.current.offsetWidth * 2) / 5,
                    ),
                );
            }
        };

        handleResize();

        window.addEventListener("resize", handleResize);
        return () => {
            window.removeEventListener("resize", handleResize);
        };
    }, [bannerContainerRef.current]);

    if (blobUrl) {
        return (
            <img
                src={blobUrl}
                alt="Banner"
                style={{
                    width: `${bannerContainerWidth}px`,
                    height: `${bannerContainerHeight}px`,
                }}
            />
        );
    } else if (banner) {
        const bannerSrc = `banners/${userId}/${banner}`;
        const animatedSrc = () => {
            if (banner.startsWith("a_")) {
                return bannerSrc.replace(".png", ".webp");
            }
            return undefined;
        };
        return (
            <Image
                src={bannerSrc}
                animated_src={animatedSrc()}
                alt="User Banner"
                width={bannerInitWidth}
                height={bannerInitHeight}
                containerWidth={bannerContainerWidth}
                containerHeight={bannerContainerHeight}
                thumbhash={thumbhash || undefined}
                optimizationType={
                    animatedSrc()
                        ? "animated_in_viewport"
                        : "static"
                }
            />
        );
    } else {
        return (
            <div
                style={{
                    backgroundColor: getIdColor(userId),
                    width: `${bannerContainerWidth}px`,
                    height: `${bannerContainerHeight}px`,
                }}
            />
        );
    }
};

const Banner = ({
    userId,
    banner,
    blobUrl,
    thumbhash,
    is_owner,
    onChange,
    containerRef,
}: BannerProps) => {
    let bannerContainerRef = useRef<HTMLDivElement>(null);

    const [editorOpen, setEditorOpen] = useState(false);
    const [dropdownOpen, setDropdownOpen] = useState(false);

    // Forward to the parent container if provided
    const handleRemove = () => {
        if (onChange) {
            onChange({
                file: undefined,
                remove: true,
                staticPreviewBlobUrl: null,
            });
        }
    };

    const handleReset = () => {
        if (onChange) {
            onChange({
                file: undefined,
                remove: undefined,
                staticPreviewBlobUrl: undefined,
            });
        }
    };

    const handleUpload = async (data: imageUploadProps) => {
        const processor = await ImageProcessor.create();

        const result = await processor.transform(
            data.File,
            {
                type: "ratio",
                width: 5,
                height: 2,
                scale: data.Scale,
            },
            {
                x: data.PositionX,
                y: data.PositionY,
            },
        );
        const banner_file = new File([result.final], data.File.name, {
            type: data.File.type,
        });
        if (onChange) {
            console.log("Banner: handleUpload", banner_file);
            onChange({
                file: banner_file,
                remove: false,
                staticPreviewBlobUrl: result.static
                    ? URL.createObjectURL(result.static)
                    : null,
            });
        }
    };

    return (
        <div className={style["banner-container"]} ref={bannerContainerRef}>
            <div className={style["banner"]}>
                <BannerPreview
                    userId={userId}
                    banner={parseStaticImage(banner ? banner : "")}
                    blobUrl={blobUrl}
                    thumbhash={thumbhash}
                    bannerContainerRef={bannerContainerRef}
                />
            </div>
            {is_owner && (
                <>
                    <ImageEditor
                        cropSelectorStyle={style["crop-selector"]}
                        ratio={[5, 2]}
                        isOpen={editorOpen}
                        onOpenChange={setEditorOpen}
                        onUpload={handleUpload}
                        containerRef={containerRef}
                    />
                    <Dropdown
                        placement="right-start"
                        open={dropdownOpen}
                        onOpenChange={setDropdownOpen}
                        offsetPlacement={5}
                    >
                        <DropdownTrigger asChild>
                            <button
                                className={`${style["edit-overlay"]} ${
                                    dropdownOpen ? style["dropdown-open"] : ""
                                }`}
                                onClick={() => setDropdownOpen(!dropdownOpen)}
                            >
                                <div className={style["edit-button"]}>
                                    <FaPen />
                                </div>
                            </button>
                        </DropdownTrigger>
                        <DropdownContent className={style["dropdown-content"]}>
                            <DropdownItem>
                                <button
                                    className={style["edit-banner-button"]}
                                    onClick={() => setEditorOpen(true)}
                                >
                                    Change Banner
                                </button>
                            </DropdownItem>
                            {blobUrl && (
                                <DropdownItem>
                                    <button
                                        className={style["reset-banner-button"]}
                                        onClick={handleReset}
                                    >
                                        Reset Banner
                                    </button>
                                </DropdownItem>
                            )}
                            <DropdownItem>
                                <button
                                    className={style["remove-banner-button"]}
                                    onClick={handleRemove}
                                >
                                    Remove Banner
                                </button>
                            </DropdownItem>
                        </DropdownContent>
                    </Dropdown>
                </>
            )}
        </div>
    );
};

export default Banner;
