import localFont from 'next/font/local';

const NerdRegular = localFont({
  src: '../../../../public/fonts/SymbolsNerdFont-Regular.ttf',
  variable: '--font-my-custom-font',
  display: 'swap',
});

const NerdMonoRegular = localFont({
  src: '../../../../public/fonts/SymbolsNerdFontMono-Regular.ttf',
  variable: '--font-my-mono-custom-font',
  display: 'swap',
});

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
      className={`${NerdRegular.className} ${NerdMonoRegular.className} ${className}`} {...props}>
      {value || children}
    </span>
  );
}
export { Icon };