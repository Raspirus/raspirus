import { serverSideTranslations } from 'next-i18next/serverSideTranslations';
import i18nextConfig from '../next-i18next.config';

/* 
* This file is used to workaround the next18 translations issue not working on static export.
* Code provided by: https://dev.to/adrai/static-html-export-with-i18n-compatibility-in-nextjs-8cd
*/

export const getI18nPaths = () =>
  i18nextConfig.i18n.locales.map((lng) => ({
    params: {
      locale: lng
    }
  }))

export const getStaticPaths = () => ({
  fallback: false,
  paths: getI18nPaths()
})

export async function getI18nProps(ctx, ns = ['common']) {
  const locale = ctx?.params?.locale
  return {
      ...(await serverSideTranslations(locale, ns))
    };
}

export function makeStaticProps(ns = {}) {
  return async function getStaticProps(ctx) {
    return {
      props: await getI18nProps(ctx, ns)
    }
  }
}
