import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import HttpBackend from 'i18next-http-backend';

// Import static translations for dev/fallback
// 导入静态翻译作为开发/后备
import en from '../locales/en/translation.json';
import zhCN from '../locales/zh-CN/translation.json';
import zhTW from '../locales/zh-TW/translation.json';
import ja from '../locales/ja/translation.json';
import ko from '../locales/ko/translation.json';

// In a real Tauri app, we might load from the filesystem using a custom backend.
// For now, we use the resources directly but structured to support loading.
const resources = {
  en: { translation: en },
  'zh-CN': { translation: zhCN },
  'zh-TW': { translation: zhTW },
  ja: { translation: ja },
  ko: { translation: ko },
};

i18n
  .use(initReactI18next) // passes i18n down to react-i18next
  .use(HttpBackend) // allows loading translations via http (if we serve them)
  .init({
    resources,
    lng: 'en', // default language
    fallbackLng: 'en',
    interpolation: {
      escapeValue: false, // react already safes from xss
    },
    // This function allows us to merge external config later
    // 此函数允许我们稍后合并外部配置
    partialBundledLanguages: true,
  });

// Mock function to simulate loading external config and merging
// 模拟加载外部配置并合并的函数
export const loadExternalConfig = async (configPath: string) => {
    try {
        console.log(`Loading external config from ${configPath}...`);
        // Logic to read JSON file from Tauri fs API would go here
        // 从 Tauri fs API 读取 JSON 文件的逻辑将放在这里
        // const externalResources = await readJson(configPath);
        // i18n.addResourceBundle('en', 'translation', externalResources.en, true, true);
    } catch (e) {
        console.warn("Failed to load external config", e);
    }
};

export default i18n;
