use std::collections::HashMap;

pub fn get_element_ui_js_tags(ts: &str) -> HashMap<String, String> {
    let mut m = HashMap::with_capacity(20);
    m.insert("el-alert".into(), format!("this.$alert('这是一段内容', '标题名称', {{\n{ts}confirmButtonText: '确定',\n{ts}callback: action => {{\n{ts}{ts}\n{ts}}}\n}})"));
    m.insert("el-conform".into(), format!("this.$confirm('此操作将永久删除该文件, 是否继续?', '提示', {{\n{ts}confirmButtonText: '确定',\n{ts}cancelButtonText: '取消',\n{ts}type: 'warning'\n}}).then(() => {{\n{ts}this.$message({{\n{ts}{ts}type: 'success',\n{ts}{ts}message: '删除成功!'\n{ts}}})\n}}).catch(() => {{\n{ts}this.$message({{\n{ts}{ts}type: 'info',\n{ts}{ts}message: '已取消删除'\n{ts}}})\n}})"));
    m.insert("el-form:clear".into(), "this.$refs['${1:form}'].clearValidate()".into());
    m.insert("el-form:valid".into(), format!("this.$refs['${{1:formName}}'].validate((valid) => {{\n{ts}if (valid) {{\n{ts}{ts}$2\n{ts}}} else {{\n{ts}{ts}return false\n{ts}}}\n}})"));
    m.insert("el-message".into(), format!("this.$message({{\n{ts}message: '恭喜你，这是一条成功消息',\n{ts}type: 'success'\n}})"));
    m.insert("el-message:close".into(), format!("this.$message({{\n{ts}message: '恭喜你，这是一条成功消息',\n{ts}showClose: true,\n{ts}type: 'success'\n}})"));
    m.insert("el-notify".into(), format!("this.$notify({{\n{ts}title: '标题名称',\n{ts}message: h('i', {{style: 'color: teal'}}, 'notify')\n}})"));
    m.insert("el-notify:noclose".into(), format!("this.$notify({{\n{ts}title: '提示',\n{ts}message: '不会自动关闭的消息',\n{ts}duration: 0\n}})"));
    m.insert("el-notify:success".into(), format!("this.$notify({{\n{ts}title: '成功',\n{ts}message: '这是一条成功的提示消息',\n{ts}type: 'success'\n}})"));
    m.insert("el-prompt".into(), format!("this.$prompt('请输入邮箱', '提示', {{\n{ts}confirmButtonText: '确定',\n{ts}cancelButtonText: '取消',\n{ts}inputPattern: '',\n{ts}inputErrorMessage: ''\n}}).then(({{ value }}) => {{\n{ts}\n}}).catch(() => {{\n{ts}\n}});"));
    m.insert("el-rules:array".into(), "{ type: 'array', required: true, message: '请至少选择一个', trigger: 'change' }".into());
    m.insert("el-rules:date".into(), "{ type: 'date', required: true, message: '请选择日期', trigger: 'change' }".into());
    m.insert("el-rules:minMax".into(), "{require: true, min: 3, max: 5, message: '长度在 3 到 5 个字符', trigger: 'blur' }".into());
    m.insert("el-rules:required".into(), "{required: true, message:'请输入', trigger: 'blur'}".into());
    m.insert("el-rules:self".into(), "{ validator: validatePass, trigger: 'blur' }".into());
    m.insert("el-rules:selfmethod".into(), format!("var validatePass = (rule, value, callback) => {{\n{ts}if (value === '') {{\n{ts}{ts}callback(new Error(''));\n{ts}}} else {{\n{ts}{ts}callback();\n{ts}}}\n}}"));
    m.insert("el-pagination".into(), format!("handleCurrentChange (pageNum) {{\n{ts}this.pageNum = pageNum\n{ts}this.fetchList()\n}},\nhandleSizeChange(pageSize) {{\n{ts}this.pageSize = pageSize\n{ts}this.fetchList()\n}}"));
    m.insert("reg-phone".into(), "/^[1][3,4,5,7,8][0-9]{9}$/".into());
    m.insert("reg-email".into(), "/^[A-Za-zd]+([-_.][A-Za-z\\d]+)*@([A-Za-z\\d]+[-.])+[A-Za-z\\d]{2,4}$/".into());
    m
}
