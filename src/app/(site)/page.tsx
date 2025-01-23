import TitleSection from "@/components/landing/title-section";
import HeroTitle from "./_components/hero-title";
import HeroInfiniteScroll from "./_components/hero-infinite-scroll";
import HeroFeatures from "./_components/hero-features";

const HomePageLayout = () => {
  return (
    <>
      <HeroTitle />
      <HeroInfiniteScroll />
      <HeroFeatures />
      <section className="relative">
        <div
          className="w-full
          blur-[120px]
          rounded-full
          h-32
          absolute
          bg-brand-primaryPurple/50
          -z-100
          top-56
        "
        />
        <div
          className="mt-20
          px-4
          sm:px-6 
          flex
          flex-col
          overflow-x-hidden
          overflow-visible
        "
        >
          <TitleSection
            title="Trusted by all"
            subheading="Join thousands of satisfied users who rely on our platform for their 
            personal and professional productivity needs."
            pill="Testimonials"
          />
        </div>
      </section>
    </>
  );
};

export default HomePageLayout;
