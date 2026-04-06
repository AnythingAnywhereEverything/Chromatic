import { useMemo } from "react"
import { cva, type VariantProps } from "class-variance-authority"

import { cn } from "@/lib/utils"
import { Label } from "./label"
import { Separator } from "./separator"

import s from "@styles/ui/Nebula/field.module.scss"

function FieldSet({ className, ...props }: React.ComponentProps<"fieldset">) {
    return (
        <fieldset
            data-component="field-set"
            className={cn(
                s.fieldSet, className)}
            {...props}
        />
    )
}

function FieldLegend({
  className,
  variant = "legend",
  ...props
}: React.ComponentProps<"legend"> & { variant?: "legend" | "label" }) {
    return (
        <legend
            data-component="field-legend"
            data-variant={variant}
            className={cn(s.fieldLegend, className)}
            {...props}
        />
    )
}

function FieldGroup({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-component="field-group"
      className={cn(
        s.fieldGroup,
        className
      )}
      {...props}
    />
  )
}

const fieldVariants = cva(s.fieldBase, {
  variants: {
    orientation: {
      vertical:
        s.vertical,
      horizontal:
        s.horizontal,
      responsive:
        s.responsive,
    },
    justify: {
      "space-between": 
        s.spacebetween,
      "space-around": 
        s.spacearound,
      "center": 
        s.center,
      none: ""
    }
  },
  defaultVariants: {
    orientation: "vertical",
    justify: "none"
  },
})

function Field({
  className,
  stretch = false,
  orientation = "vertical",
  justify = "none",
  ...props
}: React.ComponentProps<"div"> & VariantProps<typeof fieldVariants> & {stretch?: boolean}) {
  return (
    <div
      role="group"
      data-component="field"
      data-orientation={orientation}
      data-stretch={stretch}
      className={cn(fieldVariants({ orientation, justify }), className)}
      {...props}
    />
  )
}

function FieldContent({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-component="field-content"
      className={cn(
        s.fieldContent,
        className
      )}
      {...props}
    />
  )
}

function FieldLabel({
  className,
  ...props
}: React.ComponentProps<typeof Label>) {
  return (
    <Label
      data-component="field-label"
      className={cn(
        s.fieldLabel,
        className
      )}
      {...props}
    />
  )
}

function FieldTitle({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-component="field-label"
      className={cn(
        s.fieldTitle,
        className
      )}
      {...props}
    />
  )
}

function FieldDescription({ className, ...props }: React.ComponentProps<"p">) {
  return (
    <p
      data-component="field-description"
      className={cn(
        s.fieldDescription,
        className
      )}
      {...props}
    />
  )
}

function FieldSeparator({
  children,
  className,
  ...props
}: React.ComponentProps<"div"> & {
  children?: React.ReactNode
}) {
  return (
    <div
      data-component="field-separator"
      data-content={!!children}
      className={cn(s.fieldSeparatorContainer, className)}
      {...props}
    >
      <Separator className={s.separator} />
      {children && (
        <span
          className={s.content}
          data-component="field-separator-content"
        >
          {children}
        </span>
      )}
    </div>
  )
}

function FieldError({
  className,
  children,
  errors,
  ...props
}: React.ComponentProps<"div"> & {
  errors?: Array<{ message?: string } | undefined>
}) {
    const content = useMemo(() => {
        if (children) {
            return children
        }

        if (!errors?.length) {
            return null
        }

        const uniqueErrors = [
            ...new Map(errors.map((error) => [error?.message, error])).values(),
        ]

        if (uniqueErrors?.length == 1) {
            return uniqueErrors[0]?.message
        }

        return (
            <ul className={s.fieldErrorList}>
                {uniqueErrors.map(
                (error, index) =>
                    error?.message && <li key={index}>{error.message}</li>
                )}
            </ul>
        )
    }, [children, errors])

    if (!content) {
        return null
    }

    return (
        <div
            role="alert"
            data-component="field-error"
            className={cn(s.fieldError, className)}
            {...props}
        >
            {content}
        </div>
    )
}

export {
    Field,
    FieldLabel,
    FieldDescription,
    FieldError,
    FieldGroup,
    FieldLegend,
    FieldSeparator,
    FieldSet,
    FieldContent,
    FieldTitle,
}
