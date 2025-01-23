import HeroTitle from "./_components/hero-title";
import HeroInfiniteScroll from "./_components/hero-infinite-scroll";
import HeroFeatures from "./_components/hero-features";
import HeroTestimonials from "./_components/hero-testimonials";
import { HeroWorldMap } from "./_components/hero-world-map";

const HomePageLayout = () => {
  return (
    <>
      <HeroTitle />
      <HeroInfiniteScroll />
      <HeroFeatures />
      <HeroTestimonials />
      <HeroWorldMap />
    </>
  );
};

export default HomePageLayout;
