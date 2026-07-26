import * as React from "react";
import {
  useFloating,
  useClick,
  useDismiss,
  useRole,
  useListNavigation,
  useInteractions,
  FloatingFocusManager,
  offset,
  flip,
  size,
  autoUpdate,
  useMergeRefs,
  FloatingList,
  useListItem,
  useTransitionStatus,
  FloatingPortal,
  useId,
} from "@floating-ui/react";
import { useRef } from "react";

type Placement =
  | 'top'
  | 'top-start'
  | 'top-end'
  | 'right'
  | 'right-start'
  | 'right-end'
  | 'bottom'
  | 'bottom-start'
  | 'bottom-end'
  | 'left'
  | 'left-start'
  | 'left-end';

interface DropdownOptions {
    initialOpen?: boolean;
    open?: boolean;
    onOpenChange?: (open: boolean) => void;
    overlayClassName?: string;
    outsidePress?: boolean;
    placement?: Placement
    hasTypeAhead?: string[] | null;
}

function useDropdown({
    initialOpen = false,
    open: controlledOpen,
    onOpenChange: setControlledOpen,
    overlayClassName,
    outsidePress = true,
    placement = "bottom-start",
    hasTypeAhead
}: DropdownOptions = {hasTypeAhead: []}) {

  const [uncontrolledOpen, setUncontrolledOpen] = React.useState(initialOpen);
  const [labelId, setLabelId] = React.useState<string | undefined>();
  const [descriptionId, setDescriptionId] = React.useState<string | undefined>();
  
  const [activeIndex, setActiveIndex] = React.useState<number | null>(null);
  const [selectedIndex, setSelectedIndex] = React.useState<number | null>(null);
  
  const listRef = useRef([]);
  const listContentRef = React.useRef<(string | null)[]>(hasTypeAhead ?? []);
  const isTypingRef = React.useRef(false);

  const open = controlledOpen ?? uncontrolledOpen;
  const setOpen  = setControlledOpen ?? setUncontrolledOpen;
  
  const data = useFloating<HTMLElement>({
      open,
      onOpenChange: setOpen,
      placement: placement,
      whileElementsMounted: autoUpdate,
      middleware: [
        offset(5),
        flip({ padding: 10 }),
        size({
          apply({ rects, elements, availableHeight }) {
            Object.assign(elements.floating.style, {
              maxHeight: `${availableHeight}px`,
              minWidth: `${rects.reference.width}px`,
            });
          },
          padding: 10,
        }),
      ],
    })
    
    const context = data.context;

    const click = useClick(context, {
      enabled: controlledOpen == null,
      event: "mousedown",
    })
    const dismiss = useDismiss(context, {
      outsidePressEvent: "mousedown",
      outsidePress
    });
    const role = useRole(context, { role: "menu" });
    
    const listNav = useListNavigation(context, {
      listRef,
      activeIndex,
      selectedIndex,
      // setSelectedIndex,
      onNavigate: setActiveIndex,
      // This is a large list, allow looping.
      loop: true,
      focusItemOnOpen: true,
    });

    // const typeahead = useTypeahead(context, {
    //   listRef: listContentRef,
    //   activeIndex,
    //   selectedIndex,
    //   onMatch: open ? setActiveIndex : setSelectedIndex,
    //   onTypingChange(isTyping) {
    //     isTypingRef.current = isTyping;
    //   },
    // });

    const interactions = useInteractions([click, dismiss, role, listNav]);
    
    return React.useMemo(
      () => ({
        open,
        setOpen,
        ...interactions,
        ...data,
        labelId,
        descriptionId,
        setLabelId,
        setDescriptionId,
        overlayClassName,
        listRef,
        activeIndex,
        selectedIndex,
        setSelectedIndex,
      }),
      [open, setOpen, interactions,data,labelId,descriptionId,setLabelId,setDescriptionId,
        listNav, overlayClassName, listRef, activeIndex, selectedIndex, setSelectedIndex],
    );
}

type ContextType = | (ReturnType<typeof useDropdown> & {
  setLabelId: React.Dispatch<React.SetStateAction<string | undefined>>;
  setDescriptionId: React.Dispatch<
  React.SetStateAction<string | undefined>
  >;
})
| null ;

const DropdownContext  = React.createContext<ContextType>(null);
const useDropdownContext = () => {
  const context = React.useContext(DropdownContext);

  if (context == null) {
    throw new Error ("Dropdown components must be warpped in <Dropdown />")
  }
  return context;
}

function DropdownMenu ({
  children,
  ...options
}: {
  children: React.ReactNode;
} & DropdownOptions) {
  const dropdown = useDropdown(options);
  return ( 
    <DropdownContext.Provider value={dropdown}>
      {children}
    </DropdownContext.Provider>
  )
}

interface DropdownTriggerProps {
  children: React.ReactNode;
  asChild?: boolean
}

const DropdownMenuTrigger = React.forwardRef<
  HTMLElement,React.HTMLProps<HTMLElement>
  & DropdownTriggerProps>(
    function DropdownTrigger({children, asChild = false, ...props}, propRef) {
      const context = useDropdownContext();
      const childrenRef = (children as any).ref;
      const ref = useMergeRefs([context.refs.setReference, propRef, childrenRef]);

      if (asChild && React.isValidElement(children)) {
        return React.cloneElement(
          children,
          context.getReferenceProps({
            ref,
            ...props,
            ...(children.props as Record<string, unknown>),
            "data-state": context.open ? "open" : "closed",
          } as Record<string, unknown>),
        );
      }
      return(
        <button 
        ref={ref}
        type="button"
        data-state={context.open ? "open" : "closed"}
        style={{cursor: "pointer", userSelect: "none"}}
              {...context.getReferenceProps(props)}
        >{children}
        </button>
      )
    }
  )

  const DropdownMenuContent = React.forwardRef<
  HTMLDivElement,
  React.HTMLProps<HTMLDivElement>
  >(
    function DropdownMenuContent(props, propRef) {
      const {context: floatingContext, ...context} = useDropdownContext();
      const ref = useMergeRefs([context.refs.setFloating, propRef]);
      const {isMounted, status} = useTransitionStatus(floatingContext);
      
      if (!isMounted) return null;

      return(
        <FloatingPortal>
          <FloatingFocusManager context={floatingContext} modal={false}>
            <FloatingList elementsRef={context.listRef}>
                <div 
                ref={ref}
                className={context.overlayClassName}
                data-state={status}
                aria-labelledby={context.labelId}
                aria-describedby={context.descriptionId}
                  {...context.getFloatingProps(props)}
                style={{...context.floatingStyles, zIndex: 9999}}
                >
                  {props.children}
                </div>
              </FloatingList>
          </FloatingFocusManager>
        </FloatingPortal>
      )
    }
  )

  const DropdownMenuItem = React.forwardRef<HTMLDivElement,
  React.HTMLProps<HTMLDivElement> & {onSelect?: () => void} >(
    function DropdownMenuItem({ onSelect ,children, ...props}, ref) {

      const context = useDropdownContext();
      const {ref: listItemRef, index} = useListItem();
      const id = React.useId();
      const isActive = index === context.activeIndex;
      const mergedRef = useMergeRefs([listItemRef, ref]);
      return(
        <div
        id = {id}
        ref = {mergedRef}
        className={context.overlayClassName}
        role="menuitem"
        tabIndex={isActive ? 0 : -1}
        {...props}
        {...context.getItemProps({
          onClick: () => {
            context.setSelectedIndex(index);
            context.setOpen(false)
          onSelect?.();
          }
        })}
        >
          {children}
        </div>
      )
    }
  )

  const DropdownMenuGroupItem = React.forwardRef<HTMLDivElement,
  React.HTMLProps<HTMLDivElement>
  >(function DropdownMenuGroupItem ({children},ref) {
    return (
          <div role="group"
          ref={ref}
          >
            {children}
          </div>
        )
  })

  const DropdownSeperator = React.forwardRef<HTMLDivElement,
  React.HTMLProps<HTMLDivElement>
  >(function DropdownSeperator () {
    const context = useDropdownContext();
    return (
      <div 
      className={context.overlayClassName}
      role="separator">
      </div>
    )
  })
  export const DropdownPrimitive = {
    Root: DropdownMenu,
    Trigger: DropdownMenuTrigger,
    Content: DropdownMenuContent,
    Item: DropdownMenuItem,
    Group: DropdownMenuGroupItem,
    Seperator: DropdownSeperator,
  }