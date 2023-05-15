import type { GetStaticPaths, GetStaticProps } from "next";

import { Page } from "@/components/Page";
import { StatsBar } from "@/components/StatsBar";
import { API_URL } from "@/env";
import type { ApiMarket } from "@/types/api";
import React, { PropsWithChildren } from "react";

type Props = {
  marketData: ApiMarket | undefined;
  allMarketData: ApiMarket[];
};

type PathParams = {
  market_name: string;
};

const ChartCard: React.FC<PropsWithChildren<{ className?: string }>> = ({
  className,
  children,
}) => (
  <div
    className={"border border-neutral-600" + (className ? ` ${className}` : "")}
  >
    {children}
  </div>
);

const ChartName: React.FC<PropsWithChildren> = ({ children }) => (
  <p className="ml-4 mt-2 font-jost text-white">{children}</p>
);

export default function Market({ allMarketData, marketData }: Props) {
  if (!marketData) return <Page>Market not found.</Page>;

  const marketNames: string[] = allMarketData
    .sort((a, b) => a.name.localeCompare(b.name))
    .map((market) => `${market.name}`);
  return (
    <Page>
      <StatsBar marketNames={marketNames} />
      <main className="flex flex-1 gap-4 px-4 py-2">
        <div className="flex flex-1 flex-col gap-4">
          <ChartCard className="flex-1">
            <ChartName>Price Chart</ChartName>
          </ChartCard>
          <ChartCard>
            <ChartName>Orders</ChartName>
          </ChartCard>
        </div>
        <div className="flex w-[320px] flex-initial flex-col gap-4 border-neutral-600">
          <ChartCard className="flex-1">
            <ChartName>Orderbook</ChartName>
          </ChartCard>
        </div>
        <div className="flex w-[320px] flex-initial flex-col gap-4 border-neutral-600">
          <div className="flex flex-1 flex-col gap-4">
            <ChartCard className="flex-1">
              <ChartName>Order Entry</ChartName>
            </ChartCard>
            <ChartCard>
              <ChartName>Trade History</ChartName>
            </ChartCard>
          </div>
        </div>
      </main>
    </Page>
  );
}

export const getStaticPaths: GetStaticPaths<PathParams> = async () => {
  const res = await fetch(new URL("markets", API_URL).href);
  const allMarketData: ApiMarket[] = await res.json();
  const paths = allMarketData.map((market) => ({
    params: { market_name: market.name },
  }));
  return { paths, fallback: false };
};

export const getStaticProps: GetStaticProps<Props, PathParams> = async ({
  params,
}) => {
  if (!params) throw new Error("No params");
  const allMarketData: ApiMarket[] = await fetch(
    new URL("markets", API_URL).href
  ).then((res) => res.json());
  const marketData = allMarketData.find(
    (market) => market.name === params.market_name
  );

  return {
    props: {
      marketData,
      allMarketData,
    },
    revalidate: 600, // 10 minutes
  };
};