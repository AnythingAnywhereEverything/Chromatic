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
function isAnimatedImage(path: string): boolean {
  // have _a in the path
  return path.includes("a\_");
}
// * 1. if media is animated? and start with a_ and isInViewport
// * 2. look by media id which is in view so other won't get replace by current view image

// * check if vertical ratio or horizontal
// DRAFT 2
// Todo: make the image more less pixelated
// * try capped the height at 510px and crop by width?? IF it's a portrait image
interface RatioPreset {
  name: string;
  ratio: number;
  minRatio: number;
  maxRatio: number;
  class: string;
}

const RATIO_PRESETS: RatioPreset[] = [
  {
    name: "landscape",
    ratio: 16 / 9,
    minRatio: 1.2,
    maxRatio: Infinity,
    class: "aspect-video",
  },
  {
    name: "square",
    ratio: 1,
    minRatio: 0.9,
    maxRatio: 1.2,
    class: "aspect-square",
  },
  {
    name: "portrait",
    ratio: 4 / 5,
    minRatio: 0,
    maxRatio: 0.9,
    class: "aspect-portrait",
  },
];

const MAX_DISPLAY_HEIGHT = 510;
const MAX_DISPLAY_WIDTH = 540;

function pickPreset(currentRatio: number): RatioPreset {
  const match = RATIO_PRESETS.find(
    (p) => currentRatio >= p.minRatio && currentRatio < p.maxRatio,
  );
  return match ?? RATIO_PRESETS[1];
}

function ImagePortrait(width: number, height: number) {
  const currentRatio = width / height;
  const preset = pickPreset(currentRatio);
  const targetRatio = preset.ratio;

  let cropX = 0;
  let cropY = 0;
  let cropWidth = width;
  let cropHeight = height;

  if (currentRatio > targetRatio) {
    cropWidth = height * targetRatio;
    cropX = (width - cropWidth) / 2;
  } else {
    cropHeight = width / targetRatio;
    cropY = (height - cropHeight) / 2;
  }

  const renderWidth = Math.min(Math.round(cropWidth), MAX_DISPLAY_WIDTH);
  const renderHeight = Math.round(renderWidth / targetRatio);
  return {
    width: renderWidth,
    height: renderHeight,
    aspectRatioClass: preset.class,
    cropX: Math.round(cropX),
    cropY: Math.round(cropY),
    cropWidth: Math.round(cropWidth),
    cropHeight: Math.round(cropHeight),
  };
}

interface ImagePropThingy {
  width: number
  height: number
  aspectRatioClass: String
  cropX: number
  cropY: number
  cropWidth: number
  cropHeight: number
}
// * 1 collect all height media in list
// * 2 send to find their aspect ratio by one
// * 3 if large than width display slide
// * 4 else try to make it fit the image container
// ! 5 if height is the same using those height
// ! 6 if all is portrait BUT not has same height try
// ! return to 1 : 1 square ratio
// image prop for edit container
// * USING avg height on current aspect ratio
// * current ratio get by
interface GroupProp {
  avgHeight: number;
  avgWidth: number;
  presentRatio: RatioPreset;
}
function GetGroupProperty(media: MediaGroupProps): GroupProp {
  let avgHeight = 0;
  let avgWidth = 0;
  const firstImageWidth = media.media[0].width;
  const firstImageHeight = media.media[0].height;

  media.media.forEach((item) => {
    avgHeight += item.height;
    avgWidth += item.width;
  });

  avgHeight = avgHeight / media.media.length;
  avgWidth = avgWidth / media.media.length;

  const currentRatio = avgWidth / avgHeight;
  // when height equal for portrait prop
  if (avgHeight === firstImageWidth) {
    const presentRatio = pickPreset(currentRatio);
    return {
      avgHeight: firstImageHeight,
      avgWidth: avgWidth,
      presentRatio,
    };
  }

  // default by everything is 1 : 1
  const presentRatio = RATIO_PRESETS[1];
  return {
    avgHeight,
    avgWidth,
    presentRatio,
  };
}

// DRAFT 3
function SetGroupImage(media: MediaGroupProps):ImagePropThingy {
  const baseWidth = media.media[0].width;
  const baseHeight = media.media[0].height;
  let avgHeight = 0;
  let avgWidth = 0;

  media.media.map((item) => {
    avgHeight += item.height;
    avgWidth += item.width;
  });
  avgHeight = avgHeight / media.media.length;
  avgHeight = avgWidth / media.media.length;

  // is portrait
  if (avgHeight > avgWidth) {
    const currentRatio = avgHeight === baseHeight ? avgHeight / avgWidth : 1;
    const preset = pickPreset(currentRatio);
    const targetRatio = preset.ratio;

    let cropX = 0;
    let cropY = 0;
    let cropWidth = avgWidth;
    let cropHeight = avgHeight;

    if (currentRatio > targetRatio) {
      cropHeight = avgWidth * targetRatio;
      cropY = (avgHeight - cropHeight) / 2;
    } else {
      cropWidth = avgHeight / targetRatio;
      cropX = (avgWidth - cropWidth) / 2;
    }
    const renderHeight = Math.min(Math.round(cropWidth), MAX_DISPLAY_HEIGHT);
    const renderWidth = Math.round(renderHeight / targetRatio);
      return {
        width: renderWidth,
        height: renderHeight,
        aspectRatioClass: preset.class,
        cropX: Math.round(cropX),
        cropY: Math.round(cropY),
        cropWidth: Math.round(cropWidth),
        cropHeight: Math.round(cropHeight),
      };
    // Width? idk
  }
  const currentRatio = avgWidth / avgHeight ;
  const preset = pickPreset(currentRatio);
  const targetRatio = preset.ratio;
  
  let cropX = 0;
  let cropY = 0;
  let cropWidth = avgWidth;
  let cropHeight = avgHeight;
  if (currentRatio > targetRatio) {
    cropWidth = avgHeight * targetRatio;
    cropX = (avgWidth - cropWidth) / 2;
  } else {
    cropHeight = avgWidth / targetRatio;
    cropY = (avgHeight - cropHeight) / 2;
  }
  const renderWidth = Math.min(Math.round(cropWidth), MAX_DISPLAY_WIDTH);
  const renderHeight = Math.round(renderWidth / targetRatio);
  return {
    width: renderWidth,
    height: renderHeight,
    aspectRatioClass: preset.class,
    cropX: Math.round(cropX),
    cropY: Math.round(cropY),
    cropWidth: Math.round(cropWidth),
    cropHeight: Math.round(cropHeight),
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
  const size = ImagePortrait(item.width, item.height);

  return (
    <div>
      {item.mime_type?.startsWith("image/") && (
        <div
          style={{ width: size.width, height: size.height }}
          className={style["overflow-hid"]}
          key={item.id}
          ref={groupRef}
        >
          <Link href="#">
            {isInViewport && isAnimatedImage(item.path || "") ? (
              <div className={size.aspectRatioClass}>
                <ChromaImage
                  src={item.path?.replace(".png", ".webp")}
                  className={style["image-frame"]}
                  width={Math.round(size.width)}
                  height={Math.round(size.height)}
                />
              </div>
            ) : (
              <div>
                <ChromaImage
                  src={item.path}
                  thumbhash={item.thumbhash}
                  className={style["image-frame"]}
                  width={Math.round(size.width)}
                  height={Math.round(size.height)}
                />
              </div>
            )}
          </Link>
        </div>
      )}

      {item.mime_type?.startsWith("/video") && <div></div>}
    </div>
  );
};

interface ImageRatio {
  portrait: boolean;
  landscape: boolean;
  square: boolean;
}

interface MediaSize {
  width: number;
  height: number;
}

interface GroupMediaProps {
  avgHeight: number;
  avgWidth: number;
  img: ImageRatio;
}
// mediaGROUP --> check height --> get props {ratio : square or por etc.. height: }
// DRAFT 2

function ImageGroupResize(
  avgHeight: number,
  width: number,
  height: number,
  props: GroupMediaProps,
) {}

const ImageItemGroup: React.FC<{
  items: mediaPostAttechment;
  isInViewport: boolean;
  prop: ImagePropThingy;
}> = ({ items, isInViewport, prop }) => {
  return (
    <li className={style[""]}>
      <ChromaImage src={items.path} width={prop.width} height={prop.height} />
    </li>
  );
};

export const MediaGroup = ({ media }: MediaGroupProps) => {
  if (media.length === 0) {
    return null;
  }
  const [isInViewport, setIsInViewport] = React.useState(false);
  const groupRef = React.useRef<HTMLDivElement>(null);
  const property = GetGroupProperty({ media });
  const prop = SetGroupImage({ media });
  console.log(property);
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
                items={item}
                prop={prop}
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
