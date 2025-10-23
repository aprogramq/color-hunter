import type { sizeT } from "../types";

export function StartButton({height}:{height:sizeT["height"]}) {
	return (
		<box
			style={
				height > 13
					? { backgroundColor: 'white' }
					: { position: 'absolute', left: 50, top: 7, backgroundColor: 'black' }
			}
			backgroundColor={height > 13 ? 'white' : 'black'}
			padding={1}
			paddingLeft={4}
			paddingRight={4}
			marginTop={1}
		>
			<text fg={height > 13 ? '#000000' : '#ffffff'}>[S]tart</text>
		</box>
	)
}
