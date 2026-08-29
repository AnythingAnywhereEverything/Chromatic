type HlsPlayerProps = {
    // must ended with `/`
    id: string;
    base_src: string;
    thumbhash?: string;
    autoPlay?: boolean;
    controls?: boolean;
    width?: number;
    height?: number;
    duration?: number;
};

export type { HlsPlayerProps };