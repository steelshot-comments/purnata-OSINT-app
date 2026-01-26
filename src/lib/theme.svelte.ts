// src/lib/theme.svelte.ts
import { browser } from '$app/environment';

// We define the state outside the class so it's a singleton (shared everywhere)
let _mode = $state(browser ? (localStorage.getItem('theme') || 'system') : 'system');

export const themeProvider = {
    get mode() { return _mode; },
    
    setTheme(newMode: 'light' | 'dark' | 'system') {
        _mode = newMode;
        if (browser) {
            localStorage.setItem('theme', newMode);
            this.apply();
        }
    },

    apply() {
        if (!browser) return;
        
        const isDark = _mode === 'dark' || 
            (_mode === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches);
        
        // This toggles the 'dark' class on the <html> tag for Tailwind's dark: selector
        document.documentElement.classList.toggle('dark', isDark);
    }
};