import localFont from 'next/font/local';

// const NerdRegular = localFont({
//   src: '',
//   variable: '--font-my-custom-font',
//   display: 'swap',
// });

// const NerdMonoRegular = localFont({
//   src: '',
//   variable: '--font-my-mono-custom-font',
//   display: 'swap',
// });

interface IconProps {
    className?: string;
    value?: string;
    children?: React.ReactNode;
}

function Icon ({
  value,
  className, 
  children,
  ...props
}: React.ComponentProps<"span"> & IconProps) {
  return (
    <span 
      data-component="icon"
      className={`${className}`} {...props}>
      {value || children}
    </span>
  );
}
export { Icon };
