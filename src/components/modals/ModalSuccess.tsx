import { useState } from 'react'

export function ModalSuccess() {
	const [display, setDisplay] = useState<boolean>(true)
	setTimeout(() => setDisplay(false), 2000)
	if (display)
		return (
			<box backgroundColor={'#05df72'} position="absolute" right={0} width={20} height={2}>
				<text fg={'#0d542b'}>Successfully</text>
			</box>
		)
}
