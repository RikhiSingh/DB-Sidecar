"use client";

import { Button } from "@/components/ui/button";
import { ArrowRightIcon } from "lucide-react";

const Heading = () => {
  return (
    <div className="max-w-3xl space-y-4">
      <h1 className="text-3xl sm:text-5xl md:text-6xl font-bold">
        Your Ideas, Documents & Plans in One Place. Welcome to{" "}
        <span className="underline">Feytra</span>
      </h1>
      <h3 className="text-base sm:text-xl md:text-2xl font-medium">
        Feytra is the connected workspace <br />
        where better, faster work happens.
      </h3>
      <Button>
        Enter Feytra
        <ArrowRightIcon className="w-4 h-4 ml-2" />
      </Button>
    </div>
  );
};

export default Heading;
