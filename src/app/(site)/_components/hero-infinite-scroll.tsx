import { CLIENTS } from "@/lib/constants/clients";
import Image from "next/image";

const HeroInfiniteScroll = () => {
    return ( 
        <section className="relative">
        <div
          className="overflow-hidden
          flex
          after:content['']
          after:dark:from-zinc-800
          after:to-transparent
          after:from-background
          after:bg-gradient-to-l
          after:right-0
          after:bottom-0
          after:top-0
          after:w-20
          after:z-10
          after:absolute

          before:content['']
          before:dark:from-zinc-800
          before:to-transparent
          before:from-background
          before:bg-gradient-to-r
          before:left-0
          before:top-0
          before:bottom-0
          before:w-20
          before:z-10
          before:absolute
          "
        >
          {[...Array(2)].map((arr) => (
            <div key={arr} className="flex flex-no-wrap animate-slide">
              {CLIENTS.map((client) => (
                <div
                  key={client.alt}
                  className="relative w-[200px] m-20 shrink-0 flex items-center"
                >
                  <Image
                    src={client.logo}
                    alt={client.alt}
                    width={200}
                    className="object-contain max-w-none"
                  />
                </div>
              ))}
            </div>
          ))}
        </div>
      </section>
     );
}
 
export default HeroInfiniteScroll;