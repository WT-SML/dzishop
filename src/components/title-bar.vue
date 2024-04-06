<script setup lang="ts">
import { isDark, toggleDark } from 'vue-dark-switch'
import logo from '~/assets/imgs/logo.png'

const menu = [
	{
		label: '文件(F)',
		key: '文件(F)',
		submenu: [
			{
				label: '打开文件...',
				key: '打开文件...',
				accelerator: 'CmdOrCtrl+O',
			},
			{
				label: '打开文件夹...',
				key: '打开文件夹...',
				accelerator: '',
			},
			{
				label: '打开网络地址...',
				key: '打开网络地址...',
				accelerator: '',
			},
			{
				label: '导出',
				key: '导出',
				accelerator: '',
			},
			{
				label: '设置',
				key: '设置',
				accelerator: 'CmdOrCtrl+.',
			},
			{
				label: '退出',
				key: '退出',
				accelerator: '',
			},
		],
	},
	{
		label: '查看(V)',
		key: '查看(V)',
		submenu: [
			{
				label: '刷新',
				key: '刷新',
				accelerator: 'F5',
			},
			{
				label: '强制刷新',
				key: '强制刷新',
				accelerator: 'CmdOrCtrl+F5',
			},

			{ type: 'divider' },
			{
				label: '切换全屏',
				accelerator: 'F11',
				click: () => {},
			},
			{ type: 'divider' },
			{
				label: '实际大小',
				accelerator: 'CmdOrCtrl+0',
				role: 'resetzoom',
			},
			{
				label: '放大',
				accelerator: 'CmdOrCtrl+=',
				role: 'zoomin',
			},
			{
				label: '缩小',
				accelerator: 'CmdOrCtrl+-',
				role: 'zoomout',
			},

			{ type: 'divider' },
			{
				label: '开发者工具',
				role: 'toggledevtools',
				accelerator: 'F12',
			},
		],
	},
	{
		label: '主题(T)',
		submenu: [
			{
				label: '浅色主题',
				key: '浅色主题',
				onClick: () => {
					isDark.value = false
				},
			},
			{
				label: '深色主题',
				key: '深色主题',
				onClick: () => {
					isDark.value = true
				},
			},
		],
	},
	{
		label: '帮助(H)',
		submenu: [
			{
				label: '官网',
			},
			{
				label: 'Github',
			},
			{
				label: '更新日志',
			},
			// {
			// 	label: '隐私条款',
			// },
			{ type: 'divider' },
			{
				label: '检查更新...',
			},
			// {
			// 	label: '查看许可证...',
			// },
			{
				label: '关于',
			},
		],
	},
]
const handleMenySelect = (key, option) => {
	option?.onClick()
}
</script>

<template>
	<div
		data-tauri-drag-region
		:class="`h-35px bg-[#${isDark ? '21252B' : 'F8F8F8'}] flex justify-between select-none titlebar`"
	>
		<div class="h-full flex items-center pl-3">
			<img :src="logo" class="mb-2px mr-5 w-18px" />
			<div>
				<n-dropdown
					v-for="(item, i) in menu"
					:key="item.key"
					:options="item.submenu"
					trigger="click"
					placement="bottom-start"
					:animated="false"
					@select="handleMenySelect"
				>
					<div class="menu-item">{{ item.label }}</div>
				</n-dropdown>
			</div>
		</div>
		<div>
			<div id="titlebar-minimize" class="titlebar-button">
				<img
					src="https://api.iconify.design/mdi:window-minimize.svg"
					alt="minimize"
				/>
			</div>
			<div id="titlebar-maximize" class="titlebar-button">
				<img
					src="https://api.iconify.design/mdi:window-maximize.svg"
					alt="maximize"
				/>
			</div>
			<div id="titlebar-close" class="titlebar-button">
				<img src="https://api.iconify.design/mdi:close.svg" alt="close" />
			</div>
		</div>
	</div>
</template>

<style lang="scss" scoepd>
.titlebar {
	.menu-item {
		margin-right: 5px;
		display: inline-block;
		padding: 3px 6px;
		border-radius: 4px;
		&:hover {
			background-color: #eee;
		}
	}
}
.titlebar-button {
	display: inline-flex;
	justify-content: center;
	align-items: center;
	width: 35px;
	height: 35px;
	color: #fff;
}
.titlebar-button:hover {
	background: #f00;
	color: #fff !important;
}
</style>
