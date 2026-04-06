import { Separator as SeparatorPrimitive } from "radix-ui"

import { cn } from "@/lib/utils"

import s from "@styles/ui/Nebula/separator.module.scss"

function Separator({
    className,
    orientation = "horizontal",
    decorative = true,
    ...props
}: React.ComponentProps<typeof SeparatorPrimitive.Root>) {
    return (
        <SeparatorPrimitive.Root
            data-component="separator"
            decorative={decorative}
            orientation={orientation}
            className={cn(
                s.separator,
                className
            )}
            {...props}
        />
    )
}

export { Separator }
