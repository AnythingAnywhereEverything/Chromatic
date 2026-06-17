import { NextPageWithLayout } from '@/types/global';
import { GoogleOAuthProvider } from '@react-oauth/google';
import DefaultLayout from '@components/layouts/main-layouts/defaultLayout';
import '@styles/global.scss'
import { AppProps } from 'next/app';
import Head from 'next/head';
import { queryClient } from '@/hooks/clientQuery';
import { QueryClientProvider } from '@tanstack/react-query';

type AppPropsWithLayout = AppProps & {
    Component: NextPageWithLayout
}

function MyApp({ Component, pageProps }: AppPropsWithLayout) {
    const getLayout =
        Component.getLayout ??
        ((page) => <DefaultLayout>{page}</DefaultLayout>); 

    return (
        <QueryClientProvider client={queryClient}>
            <GoogleOAuthProvider clientId={process.env.NEXT_PUBLIC_GOOGLE_CLIENT_ID!}>
                <Head>
                    <title>Webapp</title>
                </Head>
                {getLayout(<Component {...pageProps} />)}
            </GoogleOAuthProvider>
        </QueryClientProvider>
    );
}

export default MyApp;