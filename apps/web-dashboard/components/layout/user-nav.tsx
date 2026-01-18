// INICIO DEL ARCHIVO [apps/web-dashboard/components/layout/user-nav.tsx]
"use client";

import { Avatar, AvatarFallback, AvatarImage } from "@radix-ui/react-avatar";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@radix-ui/react-dropdown-menu";
import { useTranslations } from "next-intl";
import { signOut } from "next-auth/react";
// ✅ FIX: Eliminados iconos no utilizados
import { LogOut, Settings, ShieldAlert } from "lucide-react";
import { useHeimdall } from "@/hooks/use-heimdall";
import { Link } from "@/lib/schemas/routing";

interface UserNavProps {
  user: {
    name?: string | null;
    email?: string | null;
    image?: string | null;
  };
}

export function UserNav({ user }: UserNavProps) {
  const t = useTranslations("Dashboard.user_nav");
  const logger = useHeimdall("UserNav");

  const initials = user.name
    ?.split(" ")
    .map((n) => n[0])
    .join("")
    .toUpperCase()
    .slice(0, 2) || "OP";

  const handleSignOut = async () => {
    logger.info(`Cerrando sesión de usuario: ${user.email}`);
    await signOut({ callbackUrl: "/" });
  };

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button
          className="relative h-10 w-10 rounded-full focus:outline-none focus:ring-2 focus:ring-primary/50 focus:ring-offset-2 focus:ring-offset-black transition-all hover:scale-105 active:scale-95 group"
          aria-label="Open user menu"
        >
          <Avatar className="h-10 w-10 rounded-full overflow-hidden border-2 border-zinc-800 group-hover:border-primary/50 transition-colors">
            <AvatarImage
              src={user.image || ""}
              alt={user.name || "Operator"}
              className="object-cover h-full w-full"
            />
            <AvatarFallback className="flex h-full w-full items-center justify-center bg-zinc-900 text-xs font-black text-zinc-400 group-hover:text-primary font-mono transition-colors">
              {initials}
            </AvatarFallback>
          </Avatar>
          <span className="absolute bottom-0 right-0 h-2.5 w-2.5 rounded-full bg-emerald-500 border-2 border-black"></span>
        </button>
      </DropdownMenuTrigger>

      <DropdownMenuContent
        className="w-72 bg-[#0C0C0C]/95 backdrop-blur-xl border border-zinc-800 text-zinc-200 p-2 shadow-[0_20px_50px_-12px_rgba(0,0,0,0.5)] rounded-2xl animate-in fade-in zoom-in-95 slide-in-from-top-3 z-50 mr-4"
        align="end"
        sideOffset={8}
        forceMount
      >
        <div className="bg-zinc-900/50 rounded-xl p-3 mb-2 border border-white/5">
          <DropdownMenuLabel className="font-normal">
            <div className="flex flex-col space-y-1">
              <p className="text-sm font-black leading-none text-white tracking-wide">
                {user.name}
              </p>
              <p className="text-[10px] leading-none text-zinc-500 font-mono truncate">
                {user.email}
              </p>
            </div>
          </DropdownMenuLabel>
        </div>

        <DropdownMenuGroup>
          <Link href="/dashboard/settings" passHref>
             <DropdownMenuItem className="cursor-pointer hover:bg-white/5 focus:bg-white/5 text-zinc-400 hover:text-white px-3 py-2.5 rounded-lg flex gap-3 text-xs items-center transition-all outline-none mb-1">
                <Settings className="w-4 h-4" />
                <span className="font-medium">{t("settings")}</span>
             </DropdownMenuItem>
          </Link>
        </DropdownMenuGroup>

        <DropdownMenuSeparator className="h-px bg-zinc-800 my-1 mx-2" />

        <DropdownMenuItem
          className="cursor-pointer bg-red-500/5 hover:bg-red-500/10 text-red-400 hover:text-red-300 focus:bg-red-500/10 px-3 py-2.5 rounded-lg flex gap-3 text-xs items-center transition-all outline-none mt-1 group"
          onClick={handleSignOut}
        >
          <LogOut className="w-4 h-4 group-hover:-translate-x-1 transition-transform" />
          <span className="font-bold tracking-wide">{t("logout")}</span>
        </DropdownMenuItem>

        <div className="px-3 py-2 mt-2 flex items-center justify-between text-[8px] text-zinc-600 font-mono uppercase tracking-widest border-t border-white/5 pt-3">
          <span className="flex items-center gap-1.5">
            <ShieldAlert className="w-3 h-3 text-emerald-900" />
            Encrypted Session
          </span>
        </div>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
// FIN DEL ARCHIVO [apps/web-dashboard/components/layout/user-nav.tsx]
