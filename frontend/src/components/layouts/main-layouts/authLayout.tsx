import { LayoutProps } from '@/types/global';

const AuthLayout: React.FC<LayoutProps> = ({ children }) => {
  return (
    <>
      <main>{children}</main>
      <footer className="footer">Standard Footer</footer>
    </>
  );
};

export default AuthLayout;