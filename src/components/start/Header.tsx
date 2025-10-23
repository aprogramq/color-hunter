import { TextAttributes } from '@opentui/core'
export function Header({ activate }:{activate:boolean}) {
	if (activate)
		return (
			<box justifyContent="center" alignItems="flex-end">
				<ascii-font font="tiny" text="Color Hunter :D" />
				<text attributes={TextAttributes.DIM} fg="#7c86ff">
					You’ll find what you need
				</text>
			</box>
		)
}
