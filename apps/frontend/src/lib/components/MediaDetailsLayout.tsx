import { Carousel } from "@mantine/carousel";
import { Text, Image, Box, Flex, Badge, Anchor, Stack } from "@mantine/core";
import { IconExternalLink } from "@tabler/icons-react";

export default function ({
	children,
	backdropImages,
	posterImages,
	externalLink,
}: {
	children: (JSX.Element | null)[];
	posterImages: string[];
	backdropImages: string[];
	externalLink: { source: string; href: string };
}) {
	return (
		<Flex direction={{ base: "column", md: "row" }} gap={"lg"}>
			<Box
				pos={"relative"}
				sx={(t) => ({
					width: "100%",
					flex: "none",
					[t.fn.largerThan("md")]: { width: "35%" },
				})}
			>
				{posterImages.length > 0 ? (
					<Carousel
						withIndicators={posterImages.length > 1}
						withControls={posterImages.length > 1}
						w={300}
					>
						{[...posterImages, ...backdropImages].map((i) => (
							<Carousel.Slide key={i}>
								<Image src={i} radius={"lg"} />
							</Carousel.Slide>
						))}
					</Carousel>
				) : (
					<Box w={300}>
						<Image withPlaceholder height={400} radius={"lg"} />
					</Box>
				)}
				<Badge
					id="data-source"
					pos={"absolute"}
					size="lg"
					top={10}
					left={10}
					color="dark"
					variant="filled"
				>
					<Flex gap={4}>
						<Text>{externalLink.source}</Text>
						<Anchor href={externalLink.href} target="_blank">
							<IconExternalLink size="1rem" />
						</Anchor>
					</Flex>
				</Badge>
			</Box>
			<Stack style={{ flexGrow: 1 }}>{children}</Stack>
		</Flex>
	);
}
