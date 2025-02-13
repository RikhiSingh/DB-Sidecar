"use client";

import { Button } from "@/components/ui/button";
import { SignInButton } from "@clerk/clerk-react";
import { useConvexAuth } from "convex/react";
import { ArrowRightIcon } from "lucide-react";
import Link from "next/link";

const Heading = () => {
  const { isAuthenticated, isLoading } = useConvexAuth();

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
      {isLoading && <p>Loading...</p>}
      {isAuthenticated && !isLoading && (
        <Button asChild>
          <Link href={"/documents"}>
            Enter Feytra
            <ArrowRightIcon className="w-4 h-4 ml-2" />
          </Link>
        </Button>
      )}
      {!isAuthenticated && !isLoading && (
        <SignInButton mode="modal">
          <Button>
            Get Feytra Free
            <ArrowRightIcon className="w-4 h-4 ml-2" />
          </Button>
        </SignInButton>
      )}
    </div>
  );
};

export default Heading;
