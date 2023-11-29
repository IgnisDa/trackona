import { conform, useForm } from "@conform-to/react";
import { Anchor, Box, Button, PasswordInput, TextInput } from "@mantine/core";
import {
	ActionFunctionArgs,
	LoaderFunctionArgs,
	MetaFunction,
	json,
} from "@remix-run/node";
import { Form, Link, useSearchParams } from "@remix-run/react";
import {
	RegisterErrorVariant,
	RegisterUserDocument,
} from "@ryot/generated/graphql/backend/graphql";
import { $path } from "remix-routes";
import { HoneypotInputs } from "remix-utils/honeypot/react";
import { match } from "ts-pattern";
import { z } from "zod";
import { getIsAuthenticated, gqlClient } from "~/lib/api.server";
import { getCoreEnabledFeatures } from "~/lib/graphql.server";
import { checkHoneypot } from "~/lib/honeypot.server";
import { createToastHeaders, redirectWithToast } from "~/lib/toast.server";
import { processSubmission } from "~/lib/utils";
import classes from "~/styles/auth.module.css";

export const redirectToQueryParam = "redirectTo";

export const loader = async ({ request }: LoaderFunctionArgs) => {
	const [isAuthenticated, _] = await getIsAuthenticated(request);
	if (isAuthenticated)
		return redirectWithToast($path("/"), {
			message: "You were already logged in",
		});
	const enabledFeatures = await getCoreEnabledFeatures();
	if (!enabledFeatures.signupAllowed)
		return redirectWithToast($path("/auth/login"), {
			message: "Registration is disabled",
			type: "error",
		});
	return json({});
};

export const meta: MetaFunction = () => [{ title: "Register | Ryot" }];

export const action = async ({ request }: ActionFunctionArgs) => {
	const formData = await request.formData();
	checkHoneypot(formData);
	const submission = processSubmission(formData, schema);
	const { registerUser } = await gqlClient.request(RegisterUserDocument, {
		input: {
			password: submission.password,
			username: submission.username,
		},
	});
	if (registerUser.__typename === "RegisterError") {
		const message = match(registerUser.error)
			.with(RegisterErrorVariant.Disabled, () => "Registration is disabled")
			.with(
				RegisterErrorVariant.UsernameAlreadyExists,
				() => "This username already exists",
			)
			.exhaustive();
		return json({ status: "error", submission } as const, {
			status: 400,
			headers: await createToastHeaders({ message, type: "error" }),
		});
	}
	return redirectWithToast($path("/auth/login"), {
		message: "Please login with your new credentials",
	});
};

const schema = z
	.object({
		username: z.string(),
		password: z
			.string()
			.min(8, "Password should be at least 8 characters long"),
		confirm: z.string(),
	})
	.refine((data) => data.password === data.confirm, {
		message: "Passwords do not match",
		path: ["confirm"],
	});

export default function Page() {
	const [searchParams] = useSearchParams();
	const [form, fields] = useForm({
		defaultValue: {
			redirectTo: searchParams.get(redirectToQueryParam) ?? "",
		},
	});

	return (
		<>
			<Box
				component={Form}
				m="auto"
				className={classes.form}
				method="post"
				{...form.props}
			>
				<HoneypotInputs />
				<TextInput
					{...conform.input(fields.username)}
					label="Username"
					autoFocus
					required
					error={fields.username.error}
				/>
				<PasswordInput
					label="Password"
					{...conform.input(fields.password)}
					mt="md"
					required
					error={fields.password.error}
				/>
				<PasswordInput
					label="Confirm password"
					mt="md"
					{...conform.input(fields.confirm)}
					required
					error={fields.confirm.error}
				/>
				<Button id="submit-button" mt="md" type="submit" w="100%">
					Register
				</Button>
				<Box mt="lg" style={{ textAlign: "right" }}>
					Already a member? Login{" "}
					<Anchor to={$path("/auth/login")} component={Link}>
						here
					</Anchor>
					.
				</Box>
			</Box>
		</>
	);
}
