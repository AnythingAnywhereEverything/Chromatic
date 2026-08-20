import { mediaPostAttechment } from "@/api/post/getFeed";
import style from "./media_group.module.scss";
import { ChromaImage } from "../chromaImage";
import React from "react";
import Link from "next/link";

interface MediaGroupProps {
  media: mediaPostAttechment[];
}

// no text will be 1:1 (ratio square)
// height 510 width will calc (when has text)

// * 2 image do it later...

function isAnimatedImage(path: string): boolean {
  // have _a in the path
  return path.includes("a\_");
}
// * 1. if media is animated? and start with a_ and isInViewport
// * 2. look by media id which is in view so other won't get replace by current view image

function calcWidthHeightSingleImg(width: number, height: number) {
    const ratio = width / height;

    // * Extreme aspect ratios get a smaller dimension so they don't dominate the feed.
    if (ratio >= 3) {
        return {
            width: 540,
            height: Math.round(540 / ratio)
        };
    }

    if (ratio <= 1 / 3) {
        return {
            width: Math.round(510 * ratio),
            height: 510
        };
    }

    const fixedHeight = 510;
    const maxWidth = 540;

    return {
        width: Math.min(Math.round(fixedHeight * ratio), maxWidth),
        height: Math.round(Math.min(fixedHeight, maxWidth / ratio))
    };
}

export const ImageSingle: React.FC<{
  item: mediaPostAttechment;
}> = ({ item }) => {
  
  const [isInViewport, setIsInViewport] = React.useState(false);
  const groupRef = React.useRef<HTMLDivElement>(null);
  React.useEffect(() => {
    const element = groupRef.current;
    if (!element) {
      return;
    }
    const observer = new IntersectionObserver(
      ([entry]) => {
        setIsInViewport(entry.isIntersecting);
      },
      {
        threshold: 0.5,
      },
    );
    observer.observe(element);

    return () => {
      observer.disconnect();
    };
  }, []);

  const size = calcWidthHeightSingleImg(item.width, item.height);
  // * How it's work, just let it work, no proper answer :D
  return (
    <div
      style={{ width: size.width, height: size.height }}
      className={style["overflow-hid"]}
    >
    <Link href={"#"}>
      <div
          style={{
            paddingBottom: `${(item.height / item.width) * 100}%`,
              width: "100%"
          }}
          >
          <div className={style["absolute-frame"]}
          ref ={groupRef}
          >
          {isInViewport && isAnimatedImage(item.path || "") ? (
            <div>
                <ChromaImage
                  src={item.path?.replace(".png",".webp")}
                  width={Math.round(size.width)}
                  height={Math.round(size.height)}
                />
              </div>
            ) : (
              <div>
                <ChromaImage
                  src={item.path}
                  className={style["image-frame"]}
                  width={Math.round(size.width)}
                  height={Math.round(size.height)}
                /></div>
            )}
          </div>
      </div>
    </Link>
  </div>
  );
};

const ImageItemGroup: React.FC<{
  items: mediaPostAttechment[];
  isInViewport: boolean;
}> = ({ items, isInViewport }) => {
  // const isAnimated = isAnimatedImage(item.path);
  // const path =
  //     isAnimated && isInViewport
  //         ? item.path.replace(".png", ".webp")
  //         : item.path;

  return (
    <ul className={style["image-grid"]}>
      {/*
            <li
                className={style["media-item"]}
                style={
                    {
                        "--aspect-ratio": `${Math.floor(item.width)} / ${Math.floor(item.height)}`,
                    } as React.CSSProperties
                }
            >
            <ChromaImage
                src={path}
                alt={item.name}
                width={Math.floor(item.width)}
                height={Math.floor(item.height)}
            />
        </li> */}
    </ul>
  );
};

export const MediaGroup = ({ media }: MediaGroupProps) => {
  if (media.length === 0) {
    return null;
  }
  const [isInViewport, setIsInViewport] = React.useState(false);
  const groupRef = React.useRef<HTMLDivElement>(null);
  React.useEffect(() => {
    const element = groupRef.current;
    if (!element) {
      return;
    }
    const observer = new IntersectionObserver(
      ([entry]) => {
        setIsInViewport(entry.isIntersecting);
      },
      {
        threshold: 0.5,
      },
    );
    observer.observe(element);

    return () => {
      observer.disconnect();
    };
  }, []);

  return (
    <div ref={groupRef} className={style["media-group"]}>
      <ul className={style["image-grid"]}>
        {/* Filter image mime out */}
        {media
          .filter((item) => item.mime_type.startsWith("image/"))
          .map((item) => {
            return (
              <ImageItemGroup
                key={item.id}
                items={media.filter((item) =>
                  item.mime_type.startsWith("image/"),
                )}
                isInViewport={isInViewport}
              />
            );
          })}
      </ul>
      <ul>
        {/* Filter video mime out */}
        {media
          .filter((item) => item.mime_type.startsWith("video/"))
          .map((item) => {
            return (
              <li key={item.id} className={style["media-item"]}>
                Waiting
              </li>
            );
          })}
      </ul>
    </div>
  );
};
