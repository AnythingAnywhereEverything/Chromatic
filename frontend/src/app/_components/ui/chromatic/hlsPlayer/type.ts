import { Media } from "@/api/types/media";

type HlsPlayerProps = {
    id: string;
    media: Media;
    width: number;
    height: number;
    containerWidth: number;
    containerHeight: number;
};
export type { HlsPlayerProps };