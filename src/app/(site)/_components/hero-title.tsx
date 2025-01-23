
import Banner from "../../../../public/app/landing/banner.png";
import TitleSection from "@/components/landing/title-section";
import { Button } from "@/components/ui/button";
import Image from "next/image";



const HeroTitle = () => {
    return ( 
        <section className="overflow-hidden px-4 sm:px-6 mt-10 sm:flex sm:flex-col gap-4 md:justify-center md:items-center">
        <TitleSection
          pill="📔 Note taking on roids!"
          title="All-In-One Collaboration and Productivity Platform"
        />
        <div className="bg-white p-[4px] mt-6 rounded-xl bg-gradient-to-r from-slate-300 to-zinc-500 sm:w-[300px]">
          <Button
            variant="secondary"
            className="w-full rounded-[10px] p-6 text-2xl pb-7"
          >
            Get Feytra Free
          </Button>
        </div>
        <div className="sm:w-[750px] flex justify-center items-center mt-[40px] relative sm:ml-0 ml-[-50px]">
          <Image src={Banner} alt="Application Banner" />
          <div className="bottom-0 top-[50%] bg-gradient-to-t dark:from-foreground left-0 right-0 absolute z--10"></div>
        </div>
      </section>
     );
}
 
export default HeroTitle;