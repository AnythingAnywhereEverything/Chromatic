import * as React from "react"
import { cn } from "@/lib/utils"
import { Button, buttonVariants } from "./button"
import {
  DayPicker,
  getDefaultClassNames,
  type DayButton,
  type Locale,
} from "react-day-picker"
import { Icon } from "./icon"
import s from "@styles/ui/Nebula/calender.module.scss"

function Calendar({
  className,
  classNames,
  showOutsideDays = true,
  captionLayout = "label",
  buttonVariant = "ghost",
  locale,
  formatters,
  components,
  ...props
}: React.ComponentProps<typeof DayPicker> & {
  buttonVariant?: React.ComponentProps<typeof Button>["variant"]
}) {
  const defaultClassNames = getDefaultClassNames()

  return (
    <DayPicker
      showOutsideDays={showOutsideDays}
      className={cn(
        s.daypicker,
        className
      )}
      captionLayout={captionLayout}
      locale={locale}
      formatters={{
        formatMonthDropdown: (date) =>
          date.toLocaleString(locale?.code, { month: "short" }),
        ...formatters,
      }}
      classNames={{
        root: cn(s.daypickerRoot, defaultClassNames.root),
        months: cn(s.daypickerMonths,
          defaultClassNames.months
        ),
        month: cn(s.daypickerMonth, defaultClassNames.month),
        nav: cn(
          s.daypickerNav,
          defaultClassNames.nav
        ),
        button_previous: cn(
          buttonVariants({ variant: buttonVariant }),
          s.daypickerButton,
          defaultClassNames.button_previous
        ),
        button_next: cn(
          buttonVariants({ variant: buttonVariant }),
          s.daypickerButton,
          defaultClassNames.button_next
        ),
        month_caption: cn(
          s.daypickerMonthCaption,
          defaultClassNames.month_caption
        ),
        dropdowns: cn(
          s.daypickerDropdowns,
          defaultClassNames.dropdowns
        ),
        dropdown_root: cn(
          s.daypickerDropdownRoot,
          defaultClassNames.dropdown_root
        ),
        dropdown: cn(
          s.daypickerDropdown,
          defaultClassNames.dropdown
        ),
        caption_label: cn(
          s.daypickerCaptionLabel,
          captionLayout === "label"
            ? ""
            : s.daypickerCaptionLabelOther,
          defaultClassNames.caption_label
        ),
        table: s.daypickerTable,
        weekdays: cn(s.daypickerWeekdays, defaultClassNames.weekdays),
        weekday: cn(
          s.daypickerWeekday,
          defaultClassNames.weekday
        ),
        week: cn(s.daypickerWeek, defaultClassNames.week),
        week_number_header: cn(
          s.daypickerWeekNumberHeader,
          defaultClassNames.week_number_header
        ),
        week_number: cn(
          s.daypickerWeekNumber,
          defaultClassNames.week_number
        ),
        day: cn(
          s.daypickerDay,
          props.showWeekNumber
            ? s.daypickerDayShowed
            : s.daypickerDayUnshowed,
          defaultClassNames.day
        ),
        range_start: cn(
          s.daypickerRange,
          s.rangeStart,
          defaultClassNames.range_start
        ),
        range_middle: cn(s.rangeMiddle, defaultClassNames.range_middle),
        range_end: cn(
          s.daypickerRange,
          s.rangeEnd,
          defaultClassNames.range_end
        ),
        today: cn(
          s.daypickerToday,
          defaultClassNames.today
        ),
        outside: cn(
          s.daypickerOutside,
          defaultClassNames.outside
        ),
        disabled: cn(
          s.daypickerDisabled,
          defaultClassNames.disabled
        ),
        hidden: cn(s.hidden, defaultClassNames.hidden),
        ...classNames,
      }}
      components={{
        Root: ({ className, rootRef, ...props }) => {
          return (
            <div
              data-component="calendar"
              ref={rootRef}
              className={cn(className)}
              {...props}
            />
          )
        },
        Chevron: ({ className, orientation, ...props }) => {
          if (orientation === "left") {
            return (
              <Icon value=""
                className={cn(s.navButton, className)}
                {...props}
              />
            )
          }

          if (orientation === "right") {
            return (
              <Icon value=""
                className={cn(s.navButton, className)}
                {...props}
              />
            )
          }

          return (
            <Icon value="" className={cn(s.navButton, className)} {...props} />
          )
        },
        DayButton: ({ ...props }) => (
          <CalendarDayButton locale={locale} {...props} />
        ),
        WeekNumber: ({ children, ...props }) => {
          return (
            <td {...props}>
              <div className="flex size-(--cell-size) items-center justify-center text-center">
                {children}
              </div>
            </td>
          )
        },
        ...components,
      }}
      {...props}
    />
  )
}

function CalendarDayButton({
  className,
  day,
  modifiers,
  locale,
  ...props
}: React.ComponentProps<typeof DayButton> & { locale?: Partial<Locale> }) {
  const defaultClassNames = getDefaultClassNames()

  const ref = React.useRef<HTMLButtonElement>(null)
  React.useEffect(() => {
    if (modifiers.focused) ref.current?.focus()
  }, [modifiers.focused])

  return (
    <Button
      ref={ref}
      variant="ghost"
      size="icon"
      data-day={day.date.toLocaleDateString(locale?.code)}
      data-selected-single={
        modifiers.selected &&
        !modifiers.range_start &&
        !modifiers.range_end &&
        !modifiers.range_middle
      }
      data-range-start={modifiers.range_start}
      data-range-end={modifiers.range_end}
      data-range-middle={modifiers.range_middle}
      className={cn(
        s.calendarDayCell,
        defaultClassNames.day,
        className
      )}
      {...props}
    />
  )
}

export { Calendar, CalendarDayButton }