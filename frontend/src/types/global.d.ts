import { NextPage } from 'next';
import { FC, ReactElement, ReactNode } from 'react';

export type LayoutProps = {
  children: ReactNode;
  Sidebar?: FC;
};

export type NextPageWithLayout<P = {}, IP = P> = NextPage<P, IP> & {
  getLayout?: (page: ReactElement) => ReactNode;
};