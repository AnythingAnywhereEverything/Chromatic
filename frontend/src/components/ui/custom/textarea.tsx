import React, { useRef, useEffect, TextareaHTMLAttributes, useImperativeHandle, forwardRef } from 'react';
import style from "./scss/textarea.module.scss"
// Extend standard HTML textarea attributes for seamless integration
interface AutoHeightTextareaProps extends TextareaHTMLAttributes<HTMLTextAreaElement> {
  value: string;
}

export const AutoHeightTextarea = forwardRef<HTMLTextAreaElement, AutoHeightTextareaProps>(({ 
  value, 
  onChange, 
  ...props 
}, ref) => {
  const localRef = useRef<HTMLTextAreaElement | null>(null);
  useImperativeHandle(ref, () => localRef.current as HTMLTextAreaElement);
   
  const textareaRef = useRef<HTMLTextAreaElement | null>(null);

  const adjustHeight = () => {
    const textarea = textareaRef.current;
    if (textarea) {
      textarea.style.height = 'auto'; // Reset to find base scrollHeight
      textarea.style.height = `${textarea.scrollHeight}px`; // Apply new height
    }
  };
  const MAX_CHARACTERS = 2500;
  // Adjust height on initial mount and whenever value changes
  useEffect(() => {
    adjustHeight();
  }, [value]);
  
    const handleChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
        if (event.target.value.length > MAX_CHARACTERS) {
            event.target.value = event.target.value.slice(0, MAX_CHARACTERS);
        }

        onChange?.(event);
        adjustHeight();
    };


  return (
    <textarea
      {...props}
      ref={textareaRef}
      value={value}
      maxLength={MAX_CHARACTERS}
      onChange={handleChange}
      className={style["textarea"]}
      style={{
        resize: 'none',       // Prevents user manual resizing
        overflowY: 'hidden',  // Hides standard scrollbar
        width: '100%',
        minHeight: '100px', 
        fontSize: "1rem",   // Sets your default base height
        ...props.style,       // Allows external style overrides
      }}
    />
  );
});
