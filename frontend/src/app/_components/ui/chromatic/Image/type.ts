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

enum OptimizationType {
    // * Image is animated on hover */
    AnimatedOnHover = "animated_on_hover",
    // * Image will load static first then load animated image for first preview */
    AnimatedOnLoad = "animated_on_load",
    // * Image is animated when it enters the viewport */
    AnimatedInViewport = "animated_in_viewport",
    // * Image is static */
    Static = "static",
}

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
    };
    
export type { ImageProps };
export { OptimizationType };