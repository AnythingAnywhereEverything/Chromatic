import style from "@styles/ui/Nebula/badge.module.scss"
import { cva, type VariantProps } from "class-variance-authority"
import { getWCAGColor } from "@/lib/getWCAGcolor";
import { cn } from "@/lib/utils";

const buttonVariants = cva(style.baseBadge, {
    variants: {
        size: {
            default: style.defaultSize,
            xs: style.extraSmallSize,
            sm: style.smallSize,
            lg: style.largeSize,
        },
    },
    defaultVariants: {
        size: "default",
    },
})

export function Badge({
    className,
    color = "#ffffff",
    size = "default",
    ...props
}: React.ComponentProps<"span"> & 
    VariantProps<typeof buttonVariants> &
    {color: string}){

    return (
        <span
            data-component="badge"
            data-size={size}
            className={cn(buttonVariants({ size }), className)}
            style={
                {backgroundColor:color, color:getWCAGColor(color)}
            }
            {...props}
        />
    )
}