<script lang='ts'>
type Props = {
  path: string
}

let path: String

export const KNOWN_LANGUAGES = {
	English: 'en',
} as const

export const KNOWN_LANGUAGE_CODES = Object.values(KNOWN_LANGUAGES)

type Sidebar = Record<
	typeof KNOWN_LANGUAGE_CODES[number],
	Record<string, { text: string; link: string }[]>
>

export const SIDEBAR: Sidebar = {
	en: {
		'Section Header': [
			{ text: 'Page ${id}', link: 'en/page-${id}' },
			{ text: 'Page ${id}', link: 'en/page-${id}' },
			{ text: 'Page ${id}', link: 'en/page-${id}' },
		],
		'Another Section': [{ text: 'Page ${id}', link: 'en/page-${id}' }],
	},
}

export const langPathRegex = /\/([a-z]{2}-?[A-Z]{0,2})\//;

export function getLanguageFromURL(pathname: string) {
	const langCodeMatch = pathname.match(langPathRegex);
	const langCode = langCodeMatch ? langCodeMatch[1] : 'en';
	return langCode as typeof KNOWN_LANGUAGE_CODES[number];
}
</script>