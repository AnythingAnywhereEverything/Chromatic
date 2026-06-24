import { LayoutProps } from '@/types/global';
import { Navbar } from '../navbarWraper';

const AuthLayout: React.FC<LayoutProps> = ({ children }) => {
  return (
    <>
      <Navbar type="auth"/>
      <main>{children}</main>
      <footer className="footer">Standard Footer</footer>
    </>
  );
};

export default AuthLayout;