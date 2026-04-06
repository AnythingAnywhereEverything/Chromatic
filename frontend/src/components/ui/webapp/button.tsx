import style from "@styles/ui/Nebula/button.module.scss"
import { cva, type VariantProps } from "class-variance-authority"
import { Slot } from "radix-ui";
import { cn } from "@/lib/utils";

export const buttonVariants = cva(style.baseButton, {
    variants: {
        variant: {
            default: style.defaultVariant,
            oppose: style.opposeVariant,
            outline: style.outlineVariant,
            secondary: style.secondaryVariant,
            ghost: style.ghostVariant,
            destructive: style.destructiveVariant,
            link: style.linkVariant,
        },
        size: {
            default: style.defaultSize,
            xs: style.extraSmallSize,
            sm: style.smallSize,
            lg: style.largeSize,
            icon: style.defaultIcon,
            "icon-xs": style.extraSmallIcon,
            "icon-sm": style.smallIcon,
            "icon-lg": style.largeIcon,
        },
        justify: {
            start: style.start,
            center: style.center,
            end: style.end,
        }
    },
    defaultVariants: {
        variant: "default",
        size: "default",
    },
})

export function Button({
    className,
    notification = 0,
    variant = "default",
    size = "default",
    justify = "center",
    asChild,
    ...props
}: React.ComponentProps<"button"> & VariantProps<typeof buttonVariants> &{
    notification?: number,
    asChild?: boolean 
}) {

    const Comp = asChild ? Slot.Root : "button"


    return (
        <Comp
            data-variant={variant}
            data-size={size}
            className={cn(buttonVariants({ variant, size, justify }), className)}
            {...props}
        />
    )
}