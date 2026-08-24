import { useState, useRef, useEffect } from "react";
import style from "./backdrop.module.scss";
import { Image } from "../../Image";
import { OptimizationType } from "../../Image/type";
interface BannerProps {
    userId: string;
    banner: string | null;
    blobUrl: string | null;
    thumbhash: string | null;
}

function parseStaticImage (url: string ) {
    // remove the extension from the url
    if (url.startsWith("a_")) {
        const urlWithoutExtension = url.replace(/\.[^/.]+$/, "");
        return `${urlWithoutExtension}.png`;
    }
    return url;
}

const BannerBackdrop = ({
    userId,
    banner,
    blobUrl,
    thumbhash,
}: BannerProps) => {
    const [isInit, setIsInit] = useState(false);
    const [bannerInitWidth, setBannerInitWidth] = useState(600);
    const [bannerInitHeight, setBannerInitHeight] = useState(240);
    const [bannerContainerWidth, setBannerContainerWidth] = useState(600);
    const [bannerContainerHeight, setBannerContainerHeight] = useState(240);
    let bannerContainerRef = useRef<HTMLDivElement>(null);

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
    }, [bannerContainerRef]);

    if (!banner && !blobUrl) {
        return null;
    }

    return (
        <div ref={bannerContainerRef} className={style["backdrop-container"]}>
            {blobUrl ? (
                <img
                    className={style["backdrop-banner"]}
                    src={blobUrl}
                    alt="Banner"
                    style={{
                        width: bannerInitWidth,
                        height: bannerInitHeight,
                    }}
                />
            ) : (
                <Image
                    className={style["backdrop-banner"]}
                    src={`banners/${userId}/${parseStaticImage(banner ? banner : "")}`}
                    width={bannerInitWidth}
                    height={bannerInitHeight}
                    containerWidth={bannerContainerWidth}
                    containerHeight={bannerContainerHeight}
                    thumbhash={thumbhash || undefined}
                    optimizationType="static"
                />
            )}
        </div>
    );
};

export default BannerBackdrop;
