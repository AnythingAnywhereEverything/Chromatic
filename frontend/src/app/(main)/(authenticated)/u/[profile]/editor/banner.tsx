import style from "./banner.module.scss";
import getIdColor from "@lib/getIdColor";
import React from "react";
import { Image } from "@/app/_components/ui/chromatic/Image";

interface BannerProps {
    userId: string;
    banner: string | null;
    banner_thumbhash: string | null;
}

function isBannerAnimated(banner: string): boolean {
    // prefix with "a_" indicates animated banner
    return banner.startsWith("a_");
}

export function Banner({ userId, banner, banner_thumbhash }: BannerProps) {
    const bannerContainerRef = React.useRef<HTMLDivElement>(null);

    const [bannerInitWidth, setBannerInitWidth] = React.useState(600);
    const [bannerInitHeight, setBannerInitHeight] = React.useState(240);

    const [bannerContainerWidth, setBannerContainerWidth] = React.useState(600);
    const [bannerContainerHeight, setBannerContainerHeight] =
        React.useState(240);

    React.useEffect(() => {
        const element = bannerContainerRef.current;

        if (!element) return;

        const handleResize = () => {
            const width = Math.min(900, Math.floor(element.offsetWidth));
            const height = Math.min(360, Math.floor((width / 5) * 2));

            setBannerContainerWidth(width);
            setBannerContainerHeight(height);

            // Only set the initial dimensions once
            setBannerInitWidth((prev) => (prev === 600 ? width : prev));
            setBannerInitHeight((prev) => (prev === 240 ? height : prev));
        };

        // Get initial dimensions
        handleResize();

        const resizeObserver = new ResizeObserver(handleResize);
        resizeObserver.observe(element);

        return () => {
            resizeObserver.disconnect();
        };
    }, []);
    return (
        <div
            ref={bannerContainerRef}
            className={style["profile-banner-container"]}
        >
            {banner ? (
                <Image
                    className={style["profile-banner"]}
                    src={`banners/${userId}/${banner}`}
                    animated_src={
                        isBannerAnimated(banner)
                            ? `banners/${userId}/${banner.replace(".png", ".webp")}`
                            : undefined
                    }
                    optimizationType="animated_in_viewport"
                    alt="User Banner"
                    width={bannerInitWidth}
                    height={bannerInitHeight}
                    containerWidth={bannerContainerWidth}
                    containerHeight={bannerContainerHeight}
                    thumbhash={banner_thumbhash || undefined}
                />
            ) : (
                <div
                    className={style["default-banner"]}
                    style={{
                        backgroundColor: getIdColor(userId),
                        width: `${bannerContainerWidth}px`,
                        height: `${bannerContainerHeight}px`,
                    }}
                />
            )}
        </div>
    );
}
