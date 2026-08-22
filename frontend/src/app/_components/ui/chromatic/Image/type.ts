type UrlParameters = {
    // * Instructed width of the image */
    width?: number;
    // * Instructed height of the image */
    height?: number;
    // * Instructed format of the image */
    format?: string;
    // * Instructed size of the image */
    size?: number;
};

type OptimizationType =
    | "animated_on_hover"
    | "animated_on_load"
    | "animated_in_viewport"
    | "static";

type ImageProps = UrlParameters &
    React.ImgHTMLAttributes<HTMLImageElement> & {
        src: string;
        animated_src?: string;
        thumbhash?: string;

        // * Container Configs */
        // * Useful for load image once and resize the container without reloading the image */
        containerClassName?: string;
        containerWidth?: number;
        containerHeight?: number;
        // * Delay in milliseconds for the transition from thumbhash to full image */
        delay?: number;
        optimizationType?: OptimizationType;
        viewportThreshold?: number;
    };
    
export type { ImageProps, OptimizationType, UrlParameters };