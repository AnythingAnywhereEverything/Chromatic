import { Dialog as DialogPrimitive } from "radix-ui"
import { cn } from "@/lib/utils"
import { Button, Icon } from "@components/ui/chromaticUI"
import { IoClose } from "react-icons/io5";
import s from "@styles/ui/Chormatic/dialogue.module.scss"

function Dialog({
  ...props
}: React.ComponentProps<typeof DialogPrimitive.Root>) {
  return <DialogPrimitive.Root data-component="dialog" {...props} />
}
function DialogTrigger({
  ...props
}: React.ComponentProps<typeof DialogPrimitive.Trigger>) {
  return <DialogPrimitive.Trigger data-component="dialog-trigger" {...props} />
}
function DialogPortal({
  ...props
}: React.ComponentProps<typeof DialogPrimitive.Portal>) {
  return <DialogPrimitive.Portal data-component="dialog-portal" {...props} />
}
function DialogClose({
  ...props
}: React.ComponentProps<typeof DialogPrimitive.Close>) {
  return <DialogPrimitive.Close data-component="dialog-close" {...props} />
}
function DialogOverlay({
  className,
  ...props
}: React.ComponentProps<typeof DialogPrimitive.Overlay>) {
  return (
    <DialogPrimitive.Overlay
      data-component="dialog-overlay"
      className={cn(s.overlay, className)}
      {...props}
    />
  )
}
function DialogContent({
  className,
  children,
  showCloseButton = true,
  noPortal = false,
  ...props
}: React.ComponentProps<typeof DialogPrimitive.Content> & {
  showCloseButton?: boolean
  noPortal?: boolean
}) {
  if (noPortal) return (
    <>
      <DialogOverlay />
      <DialogPrimitive.Content
        data-component="dialog-content"
        className={cn(
          s.dialogContent,
          className
        )}
        {...props}
      >
        {children}
        {showCloseButton && (
          <DialogPrimitive.Close data-component="dialog-close" asChild>
            <Button variant="ghost" className={s.closeBtn} size="icon-sm">
              <Icon value=""/>
              <span className={s.srOnly}>Close</span>
            </Button>
          </DialogPrimitive.Close>
        )}
      </DialogPrimitive.Content>
    </>
  )

  return (
    <DialogPortal>
      <DialogOverlay />
      <DialogPrimitive.Content
        data-component="dialog-content"
        className={cn(
          s.dialogContent,
          className
        )}
        {...props}
      >
        {children}
        {showCloseButton && (
          <DialogPrimitive.Close data-component="dialog-close" asChild>
            <Button variant="ghost" className={s.closeBtn} size="icon-sm">
              <IoClose/>
              <span className={s.srOnly}>Close</span>
            </Button>
          </DialogPrimitive.Close>
        )}
      </DialogPrimitive.Content>
    </DialogPortal>
  )
}
function DialogHeader({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-component="dialog-header"
      className={cn(s.dialogHeader, className)}
      {...props}
    />
  )
}
function DialogFooter({
  className,
  showCloseButton = false,
  children,
  ...props
}: React.ComponentProps<"div"> & {
  showCloseButton?: boolean
}) {
    return (
        <div
        data-component="dialog-footer"
        className={cn(
            s.dialogFooter,
            className
        )}
        {...props}
        >
        {children}
        {showCloseButton && (
            <DialogPrimitive.Close asChild>
            <Button variant="outline">Close</Button>
            </DialogPrimitive.Close>
        )}
        </div>
    )
}
function DialogTitle({
  className,
  ...props
}: React.ComponentProps<typeof DialogPrimitive.Title>) {
    return (
        <DialogPrimitive.Title
            data-component="dialog-title"
            className={cn(s.dialogTitle, className)}
            {...props}
        />
    )
}
function DialogDescription({
  className,
  ...props
}: React.ComponentProps<typeof DialogPrimitive.Description>) {
    return (
        <DialogPrimitive.Description
            data-component="dialog-description"
            className={cn(s.dialogDescription, className)}
            {...props}
        />
    )
}
export {
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogOverlay,
    DialogPortal,
    DialogTitle,
    DialogTrigger,
}
