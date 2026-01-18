/**
 * =================================================================
 * APARATO: PUBLIC FOOTER ZENITH (V1.1 - LINT SECURED)
 * CLASIFICACIÓN: FEATURE UI (ESTRATO L5)
 * RESPONSABILIDAD: CIERRE DE MARCA, LEGALES Y CAPTACIÓN
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementación saneada. Se eliminaron dependencias muertas de
 * animación y se reforzó el tipado de los iconos vectoriales.
 * =================================================================
 */

"use client";

import React from "react";
import { useTranslations } from "next-intl";
// ✅ FIX: Eliminado 'motion' para satisfacer TS6133
import {
  Cpu,
  MapPin,
  Mail,
  Github,
  Twitter,
  Linkedin,
  ShieldCheck,
  Zap,
  Globe,
  type LucideIcon // ✅ FIX: Importación de tipo estricto
} from "lucide-react";
import { Link } from "@/lib/schemas/routing";
import { Button } from "@/components/ui/kit/button";
import { Input } from "@/components/ui/kit/input";
import { cn } from "@/lib/utils/cn";

export function PublicFooter(): React.ReactElement {
  const t = useTranslations("PublicFooter");
  const currentYear = new Date().getFullYear();

  return (
    <footer className="relative bg-[#030303] border-t border-white/5 font-mono overflow-hidden">
      {/* Visual Background Pattern */}
      <div className="absolute inset-0 opacity-[0.02] bg-[url('https://grainy-gradients.vercel.app/noise.svg')] pointer-events-none" />

      <div className="max-w-7xl mx-auto px-6 pt-20 pb-12 relative z-10">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-12 gap-12 lg:gap-8 mb-20">

          {/* COLUMNA 1: IDENTIDAD (4 slots) */}
          <div className="lg:col-span-4 space-y-8">
            <Link href="/" className="flex items-center gap-3 group">
              <div className="h-10 w-10 bg-emerald-500/10 rounded-xl border border-emerald-500/30 flex items-center justify-center">
                <Cpu className="w-6 h-6 text-emerald-500" />
              </div>
              <span className="text-xl font-black tracking-tighter text-white uppercase italic">
                Prospector <span className="text-emerald-500">BTC</span>
              </span>
            </Link>

            <p className="text-xs text-zinc-500 leading-relaxed max-w-xs">
              {t("brand.mission")}
            </p>

            <div className="space-y-4 pt-4">
              <div className="flex items-center gap-3 text-zinc-600 hover:text-zinc-400 transition-colors">
                <MapPin className="w-4 h-4" />
                <span className="text-[10px] font-bold uppercase tracking-widest">
                  Florianópolis, SC // BRASIL
                </span>
              </div>
              <div className="flex items-center gap-4">
                <SocialIcon icon={Github} href="https://github.com/prospector-btc" />
                <SocialIcon icon={Twitter} href="#" />
                <SocialIcon icon={Linkedin} href="#" />
              </div>
            </div>
          </div>

          {/* COLUMNAS DE NAVEGACIÓN (2 slots cada una) */}
          <div className="lg:col-span-2 space-y-6">
            <h4 className="text-[10px] font-black text-white uppercase tracking-[0.3em]">{t("columns.product")}</h4>
            <ul className="space-y-4">
              <FooterLink href="/dashboard" label="Mission Control" />
              <FooterLink href="/#features" label="L1 Math Strata" />
              <FooterLink href="/#lab" label="Forensic Lab" />
            </ul>
          </div>

          <div className="lg:col-span-2 space-y-6">
            <h4 className="text-[10px] font-black text-white uppercase tracking-[0.3em]">{t("columns.resources")}</h4>
            <ul className="space-y-4">
              <FooterLink href="/blog" label="Forensic Blog" />
              <FooterLink href="/docs" label="Documentation" />
              <FooterLink href="/academy" label="Technical Academy" />
            </ul>
          </div>

          <div className="lg:col-span-4 space-y-8">
            <div className="bg-zinc-900/30 border border-white/5 p-6 rounded-3xl backdrop-blur-xl">
              <h4 className="text-xs font-black text-white uppercase tracking-widest mb-2 flex items-center gap-2">
                <Mail className="w-4 h-4 text-emerald-500" />
                {t("newsletter.title")}
              </h4>
              <p className="text-[10px] text-zinc-500 mb-6 leading-relaxed">
                {t("newsletter.description")}
              </p>
              <div className="flex gap-2">
                <Input
                  placeholder={t("newsletter.placeholder")}
                  className="bg-black/50 border-zinc-800 text-[10px] h-10"
                />
                <Button variant="cyber" className="h-10 px-4 text-[9px]">
                  {t("newsletter.button")}
                </Button>
              </div>
            </div>
          </div>
        </div>

        {/* SECTOR LEGAL Y DISCLAIMER */}
        <div className="pt-12 border-t border-white/5 flex flex-col gap-8">
          <div className="flex flex-col lg:flex-row justify-between items-start lg:items-center gap-6">
            <div className="flex flex-wrap gap-x-8 gap-y-4">
              <FooterLink href="/privacy" label="Privacy Protocol" />
              <FooterLink href="/terms" label="Usage Terms" />
              <FooterLink href="/cookies" label="Cookie Policy" />
              <FooterLink href="/affiliates" label="Affiliates" />
            </div>

            <div className="flex items-center gap-4 px-4 py-1.5 bg-zinc-900/50 rounded-full border border-white/5">
              <Globe className="w-3 h-3 text-zinc-600" />
              <span className="text-[9px] font-bold text-zinc-500 uppercase tracking-widest">
                {t("brand.location")}
              </span>
            </div>
          </div>

          <div className="p-6 bg-red-950/5 border border-red-900/20 rounded-2xl">
            <div className="flex gap-4 items-start">
              <ShieldCheck className="w-5 h-5 text-red-900 shrink-0 mt-1" />
              <p className="text-[9px] text-zinc-600 leading-relaxed font-mono italic">
                {t("disclaimer")}
              </p>
            </div>
          </div>

          <div className="flex flex-col md:flex-row justify-between items-center gap-4 pt-4 text-zinc-700">
            <span className="text-[9px] font-bold uppercase tracking-[0.3em]">
              © {currentYear} {t("brand.copyright")}
            </span>
            <div className="flex items-center gap-2">
              <Zap className="w-3 h-3 text-amber-500" />
              <span className="text-[9px] font-black uppercase tracking-widest">
                Developed by Raz Podesta for MetaShark Tech
              </span>
            </div>
          </div>
        </div>
      </div>
    </footer>
  );
}

function FooterLink({ href, label }: { href: string; label: string }) {
  return (
    <li>
      <Link
        href={href}
        // ✅ FIX: Uso de 'cn' para satisfacer TS6133
        className={cn(
          "text-[10px] text-zinc-500 hover:text-white transition-colors uppercase tracking-widest block"
        )}
      >
        {label}
      </Link>
    </li>
  );
}

// ✅ FIX: Tipado estricto 'LucideIcon'
function SocialIcon({ icon: Icon, href }: { icon: LucideIcon; href: string }) {
  return (
    <a
      href={href}
      // ✅ FIX: Uso de 'cn' para satisfacer TS6133
      className={cn(
        "p-2 bg-white/5 rounded-lg text-zinc-600 hover:text-emerald-500",
        "hover:bg-white/10 transition-all border border-transparent hover:border-emerald-500/20"
      )}
      target="_blank"
      rel="noreferrer"
    >
      <Icon className="w-4 h-4" />
    </a>
  );
}
