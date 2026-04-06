import { NextPageWithLayout } from '@/types/global';
import { GoogleOAuthProvider } from '@react-oauth/google';
import DefaultLayout from '@components/layouts/main-layouts/defaultLayout';
import '@styles/global.scss'
import { AppProps } from 'next/app';
import Head from 'next/head';

type AppPropsWithLayout = AppProps & {
    Component: NextPageWithLayout
}

function MyApp({ Component, pageProps }: AppPropsWithLayout) {
    const getLayout =
        Component.getLayout ??
        ((page) => <DefaultLayout>{page}</DefaultLayout>); 

    return (
        <>
            {/* <GoogleOAuthProvider clientId={process.env.NEXT_PUBLIC_GOOGLE_CLIENT_ID!}> */}
            <Head>
                <title></title>
            </Head>
            {getLayout(<Component {...pageProps} />)}
            {/* </GoogleOAuthProvider> */}
        </>
    );
}

export default MyApp;