import { getFormProps, getInputProps, useForm } from "@conform-to/react";
import { $path } from "@ignisda/remix-routes";
import { Anchor, Box, Button, PasswordInput, TextInput } from "@mantine/core";
import {
	type ActionFunctionArgs,
	type LoaderFunctionArgs,
	type MetaFunction,
	json,
	redirect,
} from "@remix-run/node";
import { Form, Link, useLoaderData, useSearchParams } from "@remix-run/react";
import {
	CoreDetailsDocument,
	LoginErrorVariant,
	LoginUserDocument,
} from "@ryot/generated/graphql/backend/graphql";
import { safeRedirect } from "remix-utils/safe-redirect";
import { match } from "ts-pattern";
import { z } from "zod";
import { zx } from "zodix";
import { redirectToQueryParam } from "~/lib/generals";
import {
	authCookie,
	combineHeaders,
	createToastHeaders,
	getCookiesForApplication,
	getCoreEnabledFeatures,
	getIsAuthenticated,
	gqlClient,
	processSubmission,
	redirectWithToast,
} from "~/lib/utilities.server";
import classes from "~/styles/auth.module.css";

export const loader = async ({ request }: LoaderFunctionArgs) => {
	const [isAuthenticated, _] = await getIsAuthenticated(request);
	if (isAuthenticated)
		return redirectWithToast($path("/"), {
			message: "You were already logged in",
		});
	const [enabledFeatures, { coreDetails }] = await Promise.all([
		getCoreEnabledFeatures(),
		gqlClient.request(CoreDetailsDocument),
	]);
	return json({
		enabledFeatures: { signupAllowed: enabledFeatures.signupAllowed },
		tokenValidForDays: coreDetails.tokenValidForDays,
	});
};

export const meta: MetaFunction = () => [{ title: "Login | Ryot" }];

export const action = async ({ request }: ActionFunctionArgs) => {
	const formData = await request.formData();
	const submission = processSubmission(formData, schema);
	const { loginUser } = await gqlClient.request(LoginUserDocument, {
		input: {
			password: {
				password: submission.password,
				username: submission.username,
			},
		},
	});
	if (loginUser.__typename === "LoginResponse") {
		let redirectUrl = $path("/");
		if (submission[redirectToQueryParam])
			redirectUrl = safeRedirect(submission[redirectToQueryParam]);
		const cookies = await getCookiesForApplication(loginUser.apiKey);
		const options = { maxAge: submission.tokenValidForDays * 24 * 60 * 60 };
		return redirect(redirectUrl, {
			headers: combineHeaders(
				{ "set-cookie": await authCookie.serialize(loginUser.apiKey, options) },
				cookies,
			),
		});
	}
	const message = match(loginUser.error)
		.with(
			LoginErrorVariant.CredentialsMismatch,
			() => "The password provided was incorrect",
		)
		.with(
			LoginErrorVariant.UsernameDoesNotExist,
			() => "The username provided does not exist",
		)
		.exhaustive();
	return json({ status: "error", submission, message } as const, {
		headers: await createToastHeaders({
			message,
			type: "error",
		}),
	});
};

const schema = z.object({
	username: z.string(),
	password: z.string(),
	[redirectToQueryParam]: z.string().optional(),
	tokenValidForDays: zx.NumAsString,
});

type Schema = z.infer<typeof schema>;

export default function Page() {
	const [searchParams] = useSearchParams();
	const loaderData = useLoaderData<typeof loader>();
	const [form, fields] = useForm<Schema>({});
	const redirectValue = searchParams.get(redirectToQueryParam);

	return (
		<>
			<Box
				component={Form}
				m="auto"
				className={classes.form}
				method="post"
				{...getFormProps(form)}
			>
				<input
					type="hidden"
					name="tokenValidForDays"
					value={loaderData.tokenValidForDays}
				/>
				<TextInput
					{...getInputProps(fields.username, { type: "text" })}
					label="Username"
					autoFocus
					required
				/>
				<PasswordInput
					label="Password"
					{...getInputProps(fields.password, { type: "password" })}
					mt="md"
					required
					error={fields.password.errors?.[0]}
				/>
				{redirectValue ? (
					<input
						type="hidden"
						name={redirectToQueryParam}
						value={redirectValue}
					/>
				) : null}
				<Button id="submit-button" mt="md" type="submit" w="100%">
					Login
				</Button>
				{loaderData.enabledFeatures.signupAllowed ? (
					<Box mt="lg" ta="right">
						Create a{" "}
						<Anchor to={$path("/auth/register")} component={Link}>
							new account
						</Anchor>
						?
					</Box>
				) : null}
			</Box>
		</>
	);
}
