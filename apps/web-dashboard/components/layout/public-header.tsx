/**
 * =================================================================
 * APARATO: PUBLIC HEADER SOBERANO (V1.0 - ZENITH)
 * CLASIFICACIÓN: FEATURE UI (ESTRATO L5)
 * RESPONSABILIDAD: NAVEGACIÓN PÚBLICA, AUTH GATEWAY Y A11Y
 * =================================================================
 */

"use client";

import React, { useState, useEffect } from "react";
import { useTranslations, useLocale } from "next-intl";
import { motion, AnimatePresence } from "framer-motion";
import {
  Cpu,
  Menu,
  X,
  Globe,
  ChevronDown,
  LogIn,
  UserPlus,
  Moon,
  Sun
} from "lucide-react";
import { useTheme } from "next-themes";
import { Link, useRouter, usePathname } from "@/lib/schemas/routing";
import { Button } from "@/components/ui/kit/button";
import { cn } from "@/lib/utils/cn";

export function PublicHeader(): React.ReactElement {
  const t = useTranslations();
  const locale = useLocale();
  const router = useRouter();
  const pathname = usePathname();
  const { theme, setTheme } = useTheme();

  const [is_mobile_menu_open, set_is_mobile_menu_open] = useState(false);
  const [is_scrolled, set_is_scrolled] = useState(false);

  // Sensor de Scroll para efectos de transparencia
  useEffect(() => {
    const handleScroll = () => set_is_scrolled(window.scrollY > 20);
    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  const toggle_language = () => {
    const next_locale = locale === "en" ? "es" : "en";
    router.replace(pathname, { locale: next_locale });
  };

  const nav_links = [
    { href: "#features", label: t("PublicHeader.nav.features") },
    { href: "#pricing", label: t("PublicHeader.nav.pricing") },
    { href: "#about", label: t("PublicHeader.nav.about") },
  ];

  return (
    <header
      className={cn(
        "fixed top-0 left-0 right-0 z-[100] transition-all duration-500 font-mono",
        is_scrolled
          ? "h-20 bg-black/60 backdrop-blur-2xl border-b border-white/10 shadow-2xl"
          : "h-24 bg-transparent border-b border-transparent"
      )}
    >
      <div className="max-w-7xl mx-auto h-full px-6 flex items-center justify-between">

        {/* 1. BRANDING SOBERANO */}
        <Link href="/" className="flex items-center gap-4 group">
          <div className="relative">
            <div className="absolute inset-0 bg-emerald-500/20 blur-lg rounded-xl opacity-0 group-hover:opacity-100 transition-opacity" />
            <div className="relative h-10 w-10 bg-black border border-emerald-500/30 rounded-xl flex items-center justify-center shadow-2xl group-hover:border-emerald-500 transition-colors">
              <Cpu className="h-6 w-6 text-emerald-500" />
            </div>
          </div>
          <span className="text-xl font-black tracking-tighter text-white uppercase italic">
            Prospector <span className="text-emerald-500">BTC</span>
          </span>
        </Link>

        {/* 2. NAVEGACIÓN DESKTOP (CENTER) */}
        <nav className="hidden lg:flex items-center gap-8">
          {nav_links.map((link) => (
            <Link
              key={link.href}
              href={link.href}
              className="text-[10px] font-black text-zinc-500 hover:text-white uppercase tracking-[0.3em] transition-colors relative group"
            >
              {link.label}
              <span className="absolute -bottom-1 left-0 w-0 h-px bg-emerald-500 transition-all group-hover:w-full" />
            </Link>
          ))}
        </nav>

        {/* 3. CONTROL CAPSULES (LOGIN/REGISTER/SELECTORS) */}
        <div className="hidden lg:flex items-center gap-4">

          {/* Selector de Idioma */}
          <button
            onClick={toggle_language}
            className="p-2 text-zinc-500 hover:text-white transition-colors flex items-center gap-2 border border-transparent hover:border-white/5 rounded-lg"
            aria-label="Switch Language"
          >
            <Globe className="w-4 h-4" />
            <span className="text-[10px] font-bold uppercase">{locale}</span>
          </button>

          {/* Theme Toggle */}
          <button
            onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
            className="p-2 text-zinc-500 hover:text-white transition-colors"
          >
            {theme === "dark" ? <Sun className="w-4 h-4" /> : <Moon className="w-4 h-4" />}
          </button>

          <div className="h-6 w-px bg-white/10 mx-2" />

          {/* Auth Capsules */}
          <Link href="/login">
            <Button variant="ghost" className="text-[10px] font-black tracking-widest uppercase hover:bg-white/5">
              <LogIn className="w-3.5 h-3.5 mr-2" />
              {t("PublicHeader.actions.login")}
            </Button>
          </Link>

          <Link href="/register">
            <Button variant="cyber" className="h-10 px-6 text-[10px] font-black tracking-widest">
              <UserPlus className="w-3.5 h-3.5 mr-2" />
              {t("PublicHeader.actions.get_started")}
            </Button>
          </Link>
        </div>

        {/* 4. MÓVIL: TRIGGER ACORDEÓN */}
        <button
          className="lg:hidden p-2 text-white"
          onClick={() => set_is_mobile_menu_open(!is_mobile_menu_open)}
          aria-expanded={is_mobile_menu_open}
        >
          {is_mobile_menu_open ? <X /> : <Menu />}
        </button>
      </div>

      {/* MENÚ ACORDEÓN MÓVIL (ANIMATED) */}
      <AnimatePresence>
        {is_mobile_menu_open && (
          <motion.div
            initial={{ height: 0, opacity: 0 }}
            animate={{ height: "auto", opacity: 1 }}
            exit={{ height: 0, opacity: 0 }}
            className="lg:hidden bg-[#0a0a0a] border-b border-white/10 overflow-hidden"
          >
            <div className="p-6 space-y-8">
              <div className="flex flex-col gap-6">
                {nav_links.map((link) => (
                  <Link
                    key={link.href}
                    href={link.href}
                    onClick={() => set_is_mobile_menu_open(false)}
                    className="text-sm font-bold text-zinc-400 uppercase tracking-widest flex justify-between items-center"
                  >
                    {link.label}
                    <ChevronDown className="w-4 h-4 -rotate-90 opacity-20" />
                  </Link>
                ))}
              </div>

              <div className="pt-6 border-t border-white/5 flex flex-col gap-4">
                <Link href="/login" className="w-full">
                  <Button variant="outline" className="w-full h-12 text-xs">
                    {t("PublicHeader.actions.login")}
                  </Button>
                </Link>
                <Link href="/register" className="w-full">
                  <Button variant="cyber" className="w-full h-12 text-xs">
                    {t("PublicHeader.actions.get_started")}
                  </Button>
                </Link>
              </div>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </header>
  );
}
