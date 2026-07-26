import { DropdownPrimitive } from "../../base/dropdown"
import styles from "./style.module.scss"

const Dropdown = ({
    ...props
}: React.ComponentProps<typeof DropdownPrimitive.Root>) => {
    return <DropdownPrimitive.Root data-component="dropdown" {...props} />
}

const DropdownTrigger = ({
    className,
    ...props
}: React.ComponentProps<typeof DropdownPrimitive.Trigger>) => {
    return (
        <DropdownPrimitive.Trigger data-component="dropdown-trigger"
        className={`${styles["trigger"]} ${className ?? ""}`}
        {...props} />
    )
}

const DropdownContent = ({
    className,
    ...props
}: React.ComponentProps<typeof DropdownPrimitive.Content>) => {
    return (
        <DropdownPrimitive.Content
            className={`${styles["content"]} ${className ?? ""}`}
            data-component="dropdown-content"
            {...props}
        />
    )
}

const DropdownItem = ({
    ...props
}: React.ComponentProps<typeof DropdownPrimitive.Item>) => {
    return (
        <DropdownPrimitive.Item data-component="dropdown-item" {...props} />
    )
}

const DropdownGroup = ({
    ...props
}: React.ComponentProps<typeof DropdownPrimitive.Group>) => {
    return (
        <DropdownPrimitive.Group data-component="dropdown-group" {...props} />
    )
}

const DropdownSeparator = ({
    ...props
}: React.ComponentProps<typeof DropdownPrimitive.Seperator>) => {
    return (
        <DropdownPrimitive.Seperator
            className={styles["seperator"]}
            data-component="dropdown-separator"
            {...props}
        />
    )
}

export {
    Dropdown,
    DropdownTrigger,
    DropdownContent,
    DropdownItem,
    DropdownGroup,
    DropdownSeparator,
}
