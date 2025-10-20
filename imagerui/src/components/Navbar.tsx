import { NavigationMenuItem } from "@radix-ui/react-navigation-menu"
import { NavigationMenu, NavigationMenuList } from "./ui/navigation-menu"
import { SplitText } from "@/components/shared/SplitText"
import { Link } from "@tanstack/react-router"

export const Navbar = () => {
    

    return (
        <NavigationMenu viewport={true} className="w-full max-w-none">
            <NavigationMenuList>
                <NavigationMenuItem>
                    <Link to="/">
                    <SplitText
                        text="Imager"
                        className="text-xl font-semibold text-center"
                        delay={100}
                        duration={0.6}
                        ease="power3.out"
                        splitType="chars"
                        from={{ opacity: 0, y: 40 }}
                        to={{ opacity: 1, y: 0 }}
                        threshold={0.1}
                        rootMargin="-100px"
                        textAlign="center"
                    />
                    </Link>
                </NavigationMenuItem>
            </NavigationMenuList>
        </NavigationMenu>
    )
}